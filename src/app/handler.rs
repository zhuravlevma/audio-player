use std::io::BufReader;
use std::sync::Arc;
use rodio::{OutputStream, OutputStreamHandle, PlayError, Sink, StreamError};

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_sink(&self) -> rodio::Sink {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        rodio::Sink::try_new(&handle).unwrap()
    }

    pub fn create_link_with_track(&self, path: &str) -> rodio::Sink {
        let sink = self.create_sink();
        let file = std::fs::File::open(path).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        sink
    }
}