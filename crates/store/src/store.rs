use anyhow::{anyhow, Result};
use collections::HashMap;
use futures::{
    channel::{mpsc, oneshot},
    executor::block_on,
    StreamExt,
};
use lmdb::Transaction;
use rusqlite as sqlite;
use std::{
    io::Write,
    path::{Path, PathBuf},
    thread,
};
use util::ResultExt;

// These keys receive special handling to read state from the deprecated
// sqlite database if it is present. It's a bit ugly, but a temporary stopgap
// to ensure we transfer over people's state for a few months before we remove
// the sqlite code paths entirely.
pub const DEVICE_ID: &'static [u8] = b"device_id";
pub const SHOW_UPDATE_NOTIFICATION: &'static [u8] = b"show_update_notification";
pub const OPENED_ONCE: &'static [u8] = b"opened_once";

/// Anything you want to save to the store must implement this interface
pub trait Record {
    /// The name of the entity being stored. Must be unique in the application.
    fn namespace() -> &'static str;

    /// The current version of the serialization schema.
    fn schema_version() -> u64;

    /// Turn this object into bytes conforming to the current schema version.
    fn serialize(&self) -> Vec<u8>;

    /// Turn bytes into an object for a given schema version.
    ///
    /// If you can't handle the given version, return an error.
    fn deserialize(version: u64, data: Vec<u8>) -> Result<Self>
    where
        Self: Sized;
}

/// A simple key value store for storing records with versioned serialization schemas.
#[derive(Clone)]
pub struct Store {
    request_tx: futures::channel::mpsc::UnboundedSender<Request>,
}

/// This gets used on a background thread that can block.
struct BlockingStore {
    lmdb: lmdb::Environment,
    dbs: HashMap<&'static str, lmdb::Database>,
}

/// Messages sent from the foreground to the background to avoid blocking.
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
    ReadByKey {
        namespace: &'static str,
        key: Vec<u8>,
        response: oneshot::Sender<Result<Option<(u64, Vec<u8>)>>>,
    },
    Update {
        namespace: &'static str,
        id: u64,
        version: u64,
        data: Vec<u8>,
        response: oneshot::Sender<Result<()>>,
    },
    UpdateByKey {
        namespace: &'static str,
        key: Vec<u8>,
        version: u64,
        data: Vec<u8>,
        response: oneshot::Sender<Result<()>>,
    },
    Destroy {
        namespace: &'static str,
        id: u64,
        response: oneshot::Sender<Result<()>>,
    },
    DestroyByKey {
        namespace: &'static str,
        key: Vec<u8>,
        response: oneshot::Sender<Result<()>>,
    },
}

const SEQUENCE_KEY: &'static [u8; 8] = b"sequence";

impl Store {
    pub fn new(lmdb_path: PathBuf, sqlite_path: PathBuf) -> Self {
        Self {
            request_tx: Self::spawn_background_thread(lmdb_path, sqlite_path),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn memory() -> Self {
        Self {
            request_tx: Self::spawn_memory_thread(),
        }
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

    pub async fn read_by_key<R: Record>(&self, key: Vec<u8>) -> Result<Option<R>> {
        let (tx, rx) = oneshot::channel();
        self.request_tx.unbounded_send(Request::ReadByKey {
            namespace: R::namespace(),
            key,
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

    pub async fn update_by_key<R: Record>(&self, key: Vec<u8>, record: &R) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.request_tx.unbounded_send(Request::UpdateByKey {
            namespace: R::namespace(),
            version: R::schema_version(),
            key,
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

    pub async fn destroy_by_key<R: Record>(&self, key: Vec<u8>) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.request_tx.unbounded_send(Request::DestroyByKey {
            namespace: R::namespace(),
            key,
            response: tx,
        })?;

        rx.await?
    }

    fn spawn_background_thread(
        lmdb_path: PathBuf,
        sqlite_path: PathBuf,
    ) -> mpsc::UnboundedSender<Request> {
        let (request_tx, mut request_rx) = mpsc::unbounded();

        thread::spawn(move || {
            let mut store = match BlockingStore::new(&lmdb_path, &sqlite_path) {
                Ok(store) => store,
                Err(error) => {
                    log::error!("error opening database: {}", error);
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
                        response.send(store.read(namespace, &id.to_ne_bytes())).ok();
                    }
                    Request::ReadByKey {
                        namespace,
                        key,
                        response,
                    } => {
                        response.send(store.read(namespace, &key)).ok();
                    }
                    Request::Update {
                        namespace,
                        id,
                        version,
                        data,
                        response,
                    } => {
                        response
                            .send(store.update(namespace, version, &id.to_ne_bytes(), data))
                            .ok();
                    }
                    Request::UpdateByKey {
                        namespace,
                        key,
                        version,
                        data,
                        response,
                    } => {
                        response
                            .send(store.update(namespace, version, &key, data))
                            .ok();
                    }
                    Request::Destroy {
                        namespace,
                        id,
                        response,
                    } => {
                        response
                            .send(store.destroy(namespace, &id.to_ne_bytes()))
                            .ok();
                    }
                    Request::DestroyByKey {
                        namespace,
                        key,
                        response,
                    } => {
                        response.send(store.destroy(namespace, &key)).ok();
                    }
                }
            }
        });

        request_tx
    }

    #[cfg(any(test, feature = "test-support"))]
    fn spawn_memory_thread() -> mpsc::UnboundedSender<Request> {
        let (request_tx, mut request_rx) = mpsc::unbounded();

        thread::spawn(move || {
            let mut next_id = 1_u64;
            let mut memory_store =
                HashMap::<&'static str, HashMap<Vec<u8>, (u64, Vec<u8>)>>::default();

            while let Some(request) = block_on(request_rx.next()) {
                match request {
                    Request::Create {
                        namespace,
                        version,
                        data,
                        response,
                    } => {
                        let id = next_id;
                        next_id += 1;
                        memory_store
                            .entry(namespace)
                            .or_insert_with(|| HashMap::default())
                            .insert(id.to_ne_bytes().to_vec(), (version, data));

                        response.send(Ok(id)).ok();
                    }
                    Request::Read {
                        namespace,
                        id,
                        response,
                    } => {
                        let key = id.to_ne_bytes().to_vec();
                        let entry = memory_store
                            .get(namespace)
                            .and_then(|ns| ns.get(&key).cloned());
                        response.send(Ok(entry)).ok();
                    }
                    Request::ReadByKey {
                        namespace,
                        key,
                        response,
                    } => {
                        let entry = memory_store
                            .get(namespace)
                            .and_then(|ns| ns.get(&key).cloned());
                        response.send(Ok(entry)).ok();
                    }
                    Request::Update {
                        namespace,
                        id,
                        version,
                        data,
                        response,
                    } => {
                        let key = id.to_ne_bytes().to_vec();
                        memory_store
                            .entry(namespace)
                            .or_insert_with(|| HashMap::default())
                            .insert(key, (version, data));

                        response.send(Ok(())).ok();
                    }
                    Request::UpdateByKey {
                        namespace,
                        key,
                        version,
                        data,
                        response,
                    } => {
                        memory_store
                            .entry(namespace)
                            .or_insert_with(|| HashMap::default())
                            .insert(key, (version, data));

                        response.send(Ok(())).ok();
                    }
                    Request::Destroy {
                        namespace,
                        id,
                        response,
                    } => {
                        if let Some(namespace) = memory_store.get_mut(namespace) {
                            let key = id.to_ne_bytes().to_vec();
                            namespace.remove(&key);
                        }

                        response.send(Ok(())).ok();
                    }
                    Request::DestroyByKey {
                        namespace,
                        key,
                        response,
                    } => {
                        if let Some(namespace) = memory_store.get_mut(namespace) {
                            namespace.remove(&key);
                        }

                        response.send(Ok(())).ok();
                    }
                }
            }
        });

        request_tx
    }
}

impl BlockingStore {
    fn new(lmdb_path: &Path, sqlite_path: &Path) -> Result<Self> {
        let mut builder = lmdb::Environment::new();
        builder.set_max_dbs(32);

        let mut this = Self {
            lmdb: builder.open(lmdb_path)?,
            dbs: Default::default(),
        };
        this.transfer_data_from_sqlite(sqlite_path).log_err();
        Ok(this)
    }

    fn create(&mut self, namespace: &'static str, version: u64, data: Vec<u8>) -> Result<u64> {
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

    fn read(&mut self, namespace: &'static str, key: &[u8]) -> Result<Option<(u64, Vec<u8>)>> {
        let db = self.database(namespace)?;
        let tx = self.lmdb.begin_ro_txn()?;
        let data = match tx.get(db, &key) {
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

    /// We're migrating from SQLite to LMDB, so we need to read key/value pairs from the
    /// old database and populate them in the new one the first time we start with the
    /// version of Zed containing the new database.
    fn transfer_data_from_sqlite(&mut self, db_path: &Path) -> Result<()> {
        if self.read(bool::namespace(), OPENED_ONCE)?.is_some() {
            return Ok(());
        }

        let sqlite = sqlite::Connection::open(db_path)?;
        let mut statement = sqlite.prepare("SELECT key, value FROM kv_store")?;
        let mut rows = statement.query([])?;

        while let Some(row) = rows.next()? {
            let key: String = row.get(0)?;
            let value: String = row.get(1)?;

            match key.as_str() {
                "first_open" => {
                    self.update(
                        bool::namespace(),
                        bool::schema_version(),
                        OPENED_ONCE,
                        true.serialize(),
                    )?;
                }
                "auto-updater-should-show-updated-notification" => {
                    self.update(
                        bool::namespace(),
                        bool::schema_version(),
                        SHOW_UPDATE_NOTIFICATION,
                        true.serialize(),
                    )?;
                }
                "device_id" => {
                    self.update(
                        String::namespace(),
                        String::schema_version(),
                        DEVICE_ID,
                        value.serialize(),
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn update(
        &mut self,
        namespace: &'static str,
        version: u64,
        key: &[u8],
        data: Vec<u8>,
    ) -> Result<()> {
        let db = self.database(namespace)?;
        let mut tx = self.lmdb.begin_rw_txn()?;
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

    fn destroy(&mut self, namespace: &'static str, key: &[u8]) -> Result<()> {
        let db = self.database(namespace)?;
        let mut tx = self.lmdb.begin_rw_txn()?;
        tx.del(db, &key, None)?;
        tx.commit()?;
        Ok(())
    }

    fn database(&mut self, namespace: &'static str) -> Result<lmdb::Database> {
        match self.dbs.entry(namespace) {
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
            let store = Store::new(tempdir.path().into(), PathBuf::new());

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

impl Record for String {
    fn namespace() -> &'static str {
        "String"
    }

    fn schema_version() -> u64 {
        0
    }

    fn serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn deserialize(_: u64, data: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(String::from_utf8(data)?)
    }
}

impl Record for bool {
    fn namespace() -> &'static str {
        "bool"
    }

    fn schema_version() -> u64 {
        0
    }

    fn serialize(&self) -> Vec<u8> {
        if *self {
            vec![1]
        } else {
            vec![0]
        }
    }

    fn deserialize(_: u64, data: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(data.first() == Some(&1))
    }
}

impl Record for () {
    fn namespace() -> &'static str {
        "()"
    }

    fn schema_version() -> u64 {
        0
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }

    fn deserialize(_: u64, _: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(())
    }
}
