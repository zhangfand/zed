use async_task::Runnable;

use crate::platform;

pub struct Dispatcher;

impl platform::Dispatcher for Dispatcher {
    fn is_main_thread(&self) -> bool {
        unimplemented!()
    }

    fn run_on_main_thread(&self, runnable: Runnable) {
        unimplemented!()
    }
}
