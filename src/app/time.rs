use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn time_ms_now() -> Duration {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).expect("Time went backwards")
}
pub fn get_interval_secs(start: Duration, end: Duration) -> u64 {
    end.as_secs() - start.as_secs()
}