use std::{
    ops::Deref,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use db::{query, sqlez_macros::sql};
use sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection};
use util::ResultExt;

pub(crate) struct PerfDb(pub(crate) ThreadSafeConnection<PerfDb>);

#[cfg(not(any(test, feature = "test-support")))]
pub(crate) async fn open_perf_db(db_dir: &Path) -> Option<PerfDb> {
    // TODO: Make a decision on this

    let db_path = db_dir.join(format!(
        "{}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis(),
        nanoid::nanoid!()
    ));

    ThreadSafeConnection::<PerfDb>::builder(db_path.to_string_lossy().as_ref(), true)
        .with_db_initialization_query(db::DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(db::CONNECTION_INITIALIZE_QUERY)
        .build()
        .await
        .log_err()
        .map(|connection| PerfDb(connection))
}

#[cfg(any(test, feature = "test-support"))]
pub(crate) async fn open_perf_db(db_dir: &Path) -> Option<PerfDb> {
    // TODO: Make a decision on this

    use sqlez::thread_safe_connection::locking_queue;

    let db_path = db_dir.join(format!(
        "{}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis(),
        nanoid::nanoid!()
    ));

    ThreadSafeConnection::<PerfDb>::builder(db_path.to_string_lossy().as_ref(), false)
        .with_db_initialization_query(db::DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(db::CONNECTION_INITIALIZE_QUERY)
        .with_write_queue_constructor(locking_queue())
        .build()
        .await
        .log_err()
        .map(|connection| PerfDb(connection))
}

impl Domain for PerfDb {
    fn name() -> &'static str {
        "performance"
    }

    fn migrations() -> &'static [&'static str] {
        &[sql!(
            CREATE TABLE events(
                name TEXT NOT NULL,
                span_id INTEGER,
                window_id INTEGER,
                metadata TEXT,
                nanoseconds INTEGER NOT NULL
            ) STRICT;
        )]
    }
}

impl Deref for PerfDb {
    type Target = ThreadSafeConnection<PerfDb>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PerfDb {
    query! {
        pub fn record_event(name: &'static str, span_id: Option<i64>, window_id: Option<usize>, metadata: Option<String>, nanoseconds: i64) {
            INSERT INTO events(name, span_id, window_id, metadata, nanoseconds)
            VALUES (?, ?, ?, ?, ?)
        }
    }
}
