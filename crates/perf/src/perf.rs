mod perf_util;
mod sql;

use lazy_static::lazy_static;

use crate::sql::{open_perf_db, PerfDb};
use perf_util::{get_span_id, nanoseconds_since_start, record_app_start_stamp};
use util::paths::PERF_DIR;

lazy_static! {
    static ref PERF_DB: Option<PerfDb> = smol::block_on(open_perf_db(&PERF_DIR));
}

pub struct PerfGuard {
    name: &'static str,
    span_id: i64,
    window_id: Option<usize>,
    metadata: Option<String>,
}

impl PerfGuard {
    fn new(name: &'static str, window_id: Option<usize>, metadata: Option<String>) -> PerfGuard {
        let span_id = get_span_id();
        if let Some(db) = &*PERF_DB {
            db.record_event(
                name,
                Some(span_id),
                window_id,
                metadata.clone(),
                nanoseconds_since_start(),
            );
        }
        PerfGuard {
            name,
            span_id,
            window_id,
            metadata,
        }
    }
}

impl Drop for PerfGuard {
    fn drop(&mut self) {
        if let Some(db) = &*PERF_DB {
            db.record_event(
                self.name,
                Some(self.span_id),
                self.window_id,
                self.metadata.take(),
                nanoseconds_since_start(),
            )
        }
    }
}

pub fn app_started() {
    record_app_start_stamp()
}

pub fn measure_lifetime(
    name: &'static str,
    window_id: Option<usize>,
    metadata: Option<String>,
) -> PerfGuard {
    PerfGuard::new(name, window_id, metadata)
}

pub fn measure<T>(
    name: &'static str,
    window_id: Option<usize>,
    metadata: Option<String>,
    callback: impl FnOnce() -> T,
) -> T {
    let guard = measure_lifetime(name, window_id, metadata);
    let result = callback();
    drop(guard);
    result
}

pub fn record_event(name: &'static str, window_id: Option<usize>, metadata: Option<String>) {
    if let Some(db) = &*PERF_DB {
        db.record_event(name, None, window_id, metadata, nanoseconds_since_start())
    }
}
