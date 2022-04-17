use std::sync::Arc;
use std::thread;

pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
    }

    pub fn append(&mut self, sink: Arc<rodio::Sink>) {
        thread::spawn(move || {
            sink.sleep_until_end();
        });
    }
}
