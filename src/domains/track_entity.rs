use crate::app::time::{get_interval_secs, time_ms_now};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct TrackEntity {
    track_path: String,
    time_of_start: Option<Duration>,
}

impl TrackEntity {
    pub fn new(track_path: String) -> Self {
        Self {
            track_path,
            time_of_start: Option::None,
        }
    }
    pub fn get_path(&self) -> &String {
        &self.track_path
    }

    pub fn get_start(&self) -> u64 {
        match self.time_of_start {
            None => 0,
            Some(time) => get_interval_secs(time, time_ms_now()),
        }
    }

    pub fn play(&mut self) {
        self.time_of_start = Some(Duration::from_secs(0));
    }
}
