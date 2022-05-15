use crate::modules::player::time::{get_interval_secs, time_ms_now};
use crate::modules::track::track_entity::TrackEntity;
use rodio::{OutputStream, OutputStreamHandle};
use std::io::BufReader;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Player {
    pub handle: (OutputStream, OutputStreamHandle),
    pub current_sink: Arc<rodio::Sink>,
    time_of_start: Option<Duration>,
    current_track: Option<TrackEntity>,
}

impl Player {
    pub fn new() -> Self {
        let handle = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle.1).unwrap();
        Self {
            handle,
            current_sink: Arc::new(sink),
            time_of_start: None,
            current_track: None,
        }
    }

    fn append_track(&self, track: &TrackEntity) {
        let file = std::fs::File::open(track.get_path()).unwrap();
        self.current_sink
            .append(rodio::Decoder::new(BufReader::new(file)).unwrap())
    }

    fn clear(&mut self) {
        self.current_sink.stop();
        self.current_sink = Arc::new(rodio::Sink::try_new(&self.handle.1).unwrap());
    }

    pub fn _get_time(&self) -> u64 {
        match self.time_of_start {
            None => 0,
            Some(time_of_start) => get_interval_secs(time_of_start, time_ms_now()),
        }
    }

    pub fn get_current_trackv2(&self) -> Option<&TrackEntity> {
        match &self.current_track {
            None => None,
            Some(track) => Some(track),
        }
    }

    pub fn play_track(&mut self, track: &TrackEntity) {
        self.current_track = Some(track.clone());
        self.time_of_start = Some(time_ms_now());
        self.clear();
        self.append_track(track);
        let sink = self.current_sink.clone();
        thread::spawn(move || {
            sink.sleep_until_end();
        });
    }
}
