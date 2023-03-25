use anyhow::{anyhow, Result};
use collections::HashMap;
use lmdb::Transaction;
use parking_lot::Mutex;
use std::{io::Write, path::Path};

pub trait Record {
    fn namespace() -> &'static str;
    fn schema_version() -> u64;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(version: u64, data: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

pub struct Store {
    lmdb: lmdb::Environment,
    dbs: Mutex<HashMap<&'static str, lmdb::Database>>,
}

const SEQUENCE_KEY: &'static [u8; 8] = b"sequence";

impl Store {
    pub fn new(path: &Path) -> Result<Self> {
        let mut builder = lmdb::Environment::new();
        builder.set_max_dbs(32);

        Ok(Self {
            lmdb: builder.open(path)?,
            dbs: Default::default(),
        })
    }

    pub fn create<R: Record>(&self, record: &R) -> Result<u64> {
        let db = self.database(R::namespace())?;
        let mut tx = self.lmdb.begin_rw_txn()?;

        // Compute the next id based on the previous and store it in the database.
        let id = match tx.get(db, SEQUENCE_KEY) {
            Ok(key) => u64::from_ne_bytes(key.try_into()?),
            Err(_) => 0,
        } + 1;
        let key = id.to_ne_bytes();
        tx.put(db, SEQUENCE_KEY, &key, Default::default())?;

        // Associate the record with the new id in the database. Prepend the schema version as a u64.
        let record_data = record.serialize();
        let mut buffer = tx.reserve(
            db,
            &key,
            std::mem::size_of::<u64>() + record_data.len(),
            Default::default(),
        )?;
        buffer.write_all(&R::schema_version().to_ne_bytes())?;
        buffer.write_all(&record_data)?;
        tx.commit()?;

        Ok(id)
    }

    pub fn read<R: Record>(&self, id: u64) -> Result<Option<R>> {
        let db = self.database(R::namespace())?;
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
        let record_data = &data[std::mem::size_of::<u64>()..];
        Ok(Some(R::deserialize(version, record_data)?))
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
    fn test_create() {
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

            fn deserialize(version: u64, data: &[u8]) -> Result<Self>
            where
                Self: Sized,
            {
                assert_eq!(version, Self::schema_version());
                Ok(Self(u64::from_ne_bytes(
                    data.try_into().map_err(|_| anyhow!("invalid"))?,
                )))
            }
        }

        let tempdir = TempDir::new("store_tests").unwrap();
        let store = Store::new(tempdir.path()).unwrap();

        // When key does not exist, return None.
        assert!(store.read::<Test>(1).unwrap().is_none());

        // Store a record
        let record_a1 = Test(42);
        let id = store.create(&record_a1).unwrap();
        assert_eq!(id, 1);

        // Get it back out by key. It exists
        let record_a2: Test = store.read(id).unwrap().unwrap();
        assert_eq!(record_a2, record_a1);

        // Create another record. We increment to the next id.
        let record_a1 = Test(1337);
        let id = store.create(&record_a1).unwrap();
        assert_eq!(id, 2);
    }
}
