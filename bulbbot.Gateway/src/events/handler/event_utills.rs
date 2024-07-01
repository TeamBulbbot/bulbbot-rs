use crate::events::event_handler::Handler;
use std::time::{SystemTime, UNIX_EPOCH};

impl Handler {
    pub fn get_unix_time() -> u64 {
        let start: SystemTime = SystemTime::now();
        let time_since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        time_since_epoch.as_secs()
    }
}
