use crate::app::AppError;

use std::io::{BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

pub struct Player {
    // player: rodio::Sink,
}

impl Player {
    pub fn new() -> Self {
        // let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        Self {
            // player: rodio::Sink::try_new(&handle).unwrap(),
        }
    }

    pub fn append(&mut self, sink: Arc<rodio::Sink>) {
        thread::spawn(move || {
            // sink.play();
            // sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
            sink.sleep_until_end();
        });
    }
}

pub fn play(path: &str) -> Result<(), AppError> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(path).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    let data = Arc::new(sink);
    let sink_clone = data.clone();
    let t = thread::spawn(move || {
        sink_clone.sleep_until_end();
    });
    let sink = data.clone();
    // sink.set_speed(1.2);
    sink.pause();
    sink.play();
    // sink.stop();
    t.join(); /**/
    Ok(())
}