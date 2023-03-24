use std::path::Path;

use anyhow::Result;
use collections::HashMap;
use lmdb::{Cursor, Transaction};
use parking_lot::Mutex;

pub trait Record {
    fn namespace() -> &'static str;
    fn current_version() -> u64;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(version: u64, data: Vec<u8>) -> Result<Self>
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

    pub fn create<R: Record>(&self, record: R) -> Result<u64> {
        let db = self.database(R::namespace())?;
        let mut tx = self.lmdb.begin_rw_txn()?;

        // Compute the next id based on the previous and store it in the database.
        let id = match tx.get(db, SEQUENCE_KEY) {
            Ok(key) => u64::from_ne_bytes(key.try_into()?),
            Err(_) => 0,
        } + 1;
        let key = id.to_ne_bytes();
        tx.put(db, SEQUENCE_KEY, &key, Default::default())?;

        // Associate the record with the new id in the database
        tx.put(db, &key, &record.serialize(), Default::default())?;

        Ok(id)
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
        struct Test(u64);

        impl Record for Test {
            fn namespace() -> &'static str {
                "Test"
            }

            fn current_version() -> u64 {
                0
            }

            fn serialize(&self) -> Vec<u8> {
                self.0.to_ne_bytes().to_vec()
            }

            fn deserialize(version: u64, data: Vec<u8>) -> Result<Self>
            where
                Self: Sized,
            {
                assert_eq!(version, Self::current_version());
                Ok(Self(u64::from_ne_bytes(
                    data.try_into().map_err(|_| anyhow!("invalid"))?,
                )))
            }
        }

        let tempdir = TempDir::new("store_tests").unwrap();
        let db = Store::new(tempdir.path()).unwrap();
        let id = db.create(Test(42)).unwrap();
        // assert_eq!(id, 1);
    }
}
