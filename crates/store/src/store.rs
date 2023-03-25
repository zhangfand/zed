use anyhow::{anyhow, Result};
use collections::HashMap;
use futures::{
    channel::{mpsc, oneshot},
    executor::block_on,
    StreamExt,
};
use lmdb::Transaction;
use parking_lot::Mutex;
use std::{
    io::Write,
    path::{Path, PathBuf},
    thread,
};

pub trait Record {
    fn namespace() -> &'static str;
    fn schema_version() -> u64;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(version: u64, data: Vec<u8>) -> Result<Self>
    where
        Self: Sized;
}

pub struct Store {
    request_tx: futures::channel::mpsc::UnboundedSender<Request>,
}

struct BlockingStore {
    lmdb: lmdb::Environment,
    dbs: Mutex<HashMap<&'static str, lmdb::Database>>,
}

enum Request {
    Create {
        namespace: &'static str,
        version: u64,
        data: Vec<u8>,
        response: oneshot::Sender<Result<u64>>,
    },
    Read {
        namespace: &'static str,
        id: u64,
        response: oneshot::Sender<Result<Option<(u64, Vec<u8>)>>>,
    },
    Update {
        namespace: &'static str,
        id: u64,
        version: u64,
        data: Vec<u8>,
        response: oneshot::Sender<Result<()>>,
    },
    Destroy {
        namespace: &'static str,
        id: u64,
        response: oneshot::Sender<Result<()>>,
    },
}

const SEQUENCE_KEY: &'static [u8; 8] = b"sequence";

impl Store {
    pub async fn new(path: PathBuf) -> Result<Self> {
        let request_tx = Self::spawn_background_thread(path).await?;
        Ok(Self { request_tx })
    }

    pub async fn create<R: Record>(&self, record: &R) -> Result<u64> {
        let (tx, rx) = oneshot::channel();
        self.request_tx.unbounded_send(Request::Create {
            namespace: R::namespace(),
            version: R::schema_version(),
            data: record.serialize(),
            response: tx,
        })?;

        rx.await?
    }

    pub async fn read<R: Record>(&self, id: u64) -> Result<Option<R>> {
        let (tx, rx) = oneshot::channel();
        self.request_tx.unbounded_send(Request::Read {
            namespace: R::namespace(),
            id,
            response: tx,
        })?;

        if let Some((version, data)) = rx.await?? {
            Ok(Some(R::deserialize(version, data)?))
        } else {
            Ok(None)
        }
    }

    pub async fn update<R: Record>(&self, id: u64, record: &R) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.request_tx.unbounded_send(Request::Update {
            namespace: R::namespace(),
            version: R::schema_version(),
            id,
            data: record.serialize(),
            response: tx,
        })?;

        rx.await?
    }

    pub async fn destroy<R: Record>(&self, id: u64) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.request_tx.unbounded_send(Request::Destroy {
            namespace: R::namespace(),
            id,
            response: tx,
        })?;

        rx.await?
    }

    async fn spawn_background_thread(path: PathBuf) -> Result<mpsc::UnboundedSender<Request>> {
        let (setup_tx, setup_rx) = oneshot::channel::<Result<()>>();
        let (request_tx, mut request_rx) = mpsc::unbounded();

        thread::spawn(move || {
            let store = match BlockingStore::new(&path) {
                Ok(store) => {
                    let _ = setup_tx.send(Ok(()));
                    store
                }
                Err(error) => {
                    let _ = setup_tx.send(Err(anyhow!(error)));
                    return;
                }
            };

            while let Some(request) = block_on(request_rx.next()) {
                match request {
                    Request::Create {
                        namespace,
                        version,
                        data,
                        response,
                    } => {
                        response.send(store.create(namespace, version, data)).ok();
                    }
                    Request::Read {
                        namespace,
                        id,
                        response,
                    } => {
                        response.send(store.read(namespace, id)).ok();
                    }
                    Request::Update {
                        namespace,
                        id,
                        version,
                        data,
                        response,
                    } => {
                        response
                            .send(store.update(namespace, version, id, data))
                            .ok();
                    }
                    Request::Destroy {
                        namespace,
                        id,
                        response,
                    } => {
                        response.send(store.destroy(namespace, id)).ok();
                    }
                }
            }
        });

        setup_rx.await??;
        Ok(request_tx)
    }
}

impl BlockingStore {
    fn new(path: &Path) -> Result<Self> {
        let mut builder = lmdb::Environment::new();
        builder.set_max_dbs(32);

        Ok(Self {
            lmdb: builder.open(path)?,
            dbs: Default::default(),
        })
    }

    fn create(&self, namespace: &'static str, version: u64, data: Vec<u8>) -> Result<u64> {
        let db = self.database(namespace)?;
        let mut tx = self.lmdb.begin_rw_txn()?;

        // Compute the next id based on the previous and store it in the database.
        let id = match tx.get(db, SEQUENCE_KEY) {
            Ok(key) => u64::from_ne_bytes(key.try_into()?),
            Err(_) => 0,
        } + 1;
        let key = id.to_ne_bytes();
        tx.put(db, SEQUENCE_KEY, &key, Default::default())?;

        // Associate the record with the new id in the database. Prepend the schema version as a u64.
        let mut buffer = tx.reserve(
            db,
            &key,
            std::mem::size_of::<u64>() + data.len(),
            Default::default(),
        )?;
        buffer.write_all(&version.to_ne_bytes())?;
        buffer.write_all(&data)?;
        tx.commit()?;

        Ok(id)
    }

    fn read(&self, namespace: &'static str, id: u64) -> Result<Option<(u64, Vec<u8>)>> {
        let db = self.database(namespace)?;
        let tx = self.lmdb.begin_ro_txn()?;
        let data = match tx.get(db, &id.to_ne_bytes()) {
            Ok(data) => data,
            Err(error) => {
                if error == lmdb::Error::NotFound {
                    return Ok(None);
                } else {
                    return Err(anyhow!(error));
                }
            }
        };
        let version = u64::from_ne_bytes(data[..std::mem::size_of::<u64>()].try_into()?);
        let data = data[std::mem::size_of::<u64>()..].to_vec();
        Ok(Some((version, data)))
    }

    fn update(&self, namespace: &'static str, version: u64, id: u64, data: Vec<u8>) -> Result<()> {
        let db = self.database(namespace)?;
        let mut tx = self.lmdb.begin_rw_txn()?;
        let key = id.to_ne_bytes();
        let mut buffer = tx.reserve(
            db,
            &key,
            std::mem::size_of::<u64>() + data.len(),
            Default::default(),
        )?;
        buffer.write_all(&version.to_ne_bytes())?;
        buffer.write_all(&data)?;
        tx.commit()?;

        Ok(())
    }

    fn destroy(&self, namespace: &'static str, id: u64) -> Result<()> {
        let db = self.database(namespace)?;
        let mut tx = self.lmdb.begin_rw_txn()?;
        let key = id.to_ne_bytes();
        tx.del(db, &key, None)?;
        tx.commit()?;
        Ok(())
    }

    fn database(&self, namespace: &'static str) -> Result<lmdb::Database> {
        match self.dbs.lock().entry(namespace) {
            collections::hash_map::Entry::Occupied(db) => Ok(db.get().clone()),
            collections::hash_map::Entry::Vacant(_) => {
                Ok(self.lmdb.create_db(Some(namespace), Default::default())?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use tempdir::TempDir;

    use super::*;

    #[test]
    fn test_store() {
        #[derive(Eq, PartialEq, Debug)]
        struct Test(u64);

        impl Record for Test {
            fn namespace() -> &'static str {
                "Test"
            }

            fn schema_version() -> u64 {
                0
            }

            fn serialize(&self) -> Vec<u8> {
                self.0.to_ne_bytes().to_vec()
            }

            fn deserialize(version: u64, data: Vec<u8>) -> Result<Self>
            where
                Self: Sized,
            {
                assert_eq!(version, Self::schema_version());
                Ok(Self(u64::from_ne_bytes(
                    data.try_into().map_err(|_| anyhow!("invalid"))?,
                )))
            }
        }

        block_on(async {
            let tempdir = TempDir::new("store_tests").unwrap();
            let store = Store::new(tempdir.path().into()).await.unwrap();

            // When key does not exist, return None.
            assert!(store.read::<Test>(1).await.unwrap().is_none());

            // Store a record
            let record_a1 = Test(42);
            let id_a = store.create(&record_a1).await.unwrap();
            assert_eq!(id_a, 1);

            // Get it back out by key. It exists
            let record_a2: Test = store.read(id_a).await.unwrap().unwrap();
            assert_eq!(record_a2, record_a1);

            // Create another record. We increment to the next id.
            let mut record_b1 = Test(1337);
            let id_b = store.create(&record_a1).await.unwrap();
            assert_eq!(id_b, 2);

            // Update the new record. It should change in the database.
            record_b1.0 = 1234;
            store.update(id_b, &record_b1).await.unwrap();
            let record_b2: Test = store.read(id_b).await.unwrap().unwrap();
            assert_eq!(record_b2, record_b1);

            // Destroy the first record. It is no longer in the database afterwards.
            store.destroy::<Test>(id_b).await.unwrap();
            assert!(store.read::<Test>(id_b).await.unwrap().is_none());
        });
    }
}
