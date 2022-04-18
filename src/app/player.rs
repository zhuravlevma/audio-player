use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
    }

    pub fn append(&mut self, sink: Arc<rodio::Sink>) {
        // let now = SystemTime::now();
        // println!("Player run, {:?}", ms);
        thread::spawn(move || {
            sink.sleep_until_end();
        });
    }
}
