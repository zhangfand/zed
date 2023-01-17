use std::sync::atomic::{AtomicI64, Ordering};

use quanta::Clock;

lazy_static::lazy_static! {
    static ref CLOCK: Clock = Clock::new();
    static ref APP_STARTED: u64 = CLOCK.raw();
    static ref SPAN_ID: AtomicI64 = AtomicI64::new(i64::min_value());
}

pub(crate) fn get_span_id() -> i64 {
    SPAN_ID.fetch_add(1, Ordering::Relaxed)
}

pub(crate) fn record_app_start_stamp() {
    lazy_static::initialize(&APP_STARTED);
}

pub(crate) fn nanoseconds_since_start() -> i64 {
    CLOCK.delta(*APP_STARTED, CLOCK.raw()).as_nanos() as i64
}
