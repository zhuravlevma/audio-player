use crate::app::ctx::player::time::{get_interval_secs, time_ms_now};
use crate::app::modules::track::track_entity::TrackEntity;
use rodio::{OutputStream, OutputStreamHandle};
use std::io::{BufReader, Cursor};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Player {
    pub handle: (OutputStream, OutputStreamHandle),
    pub current_sink: Arc<rodio::Sink>,
    time_of_start: Option<Duration>,
    current_track: Option<TrackEntity>,
    pub pause_time: Option<Duration>,
    pub is_empty: bool,
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
            pause_time: None,
            is_empty: true,
        }
    }

    fn append_track(&self, track: &TrackEntity) {
        let file = std::fs::File::open(track.get_path()).unwrap();
        self.current_sink
            .append(rodio::Decoder::new(BufReader::new(file)).unwrap())
    }

    async fn append_external_track(&self, track: &TrackEntity) {
        let resp = reqwest::get(track.get_path()).await.unwrap();
        let bytes = resp.bytes().await.unwrap();
        let cursor = Cursor::new(bytes); // Adds Read and Seek to the bytes via Cursor
        let source = rodio::Decoder::new(cursor).unwrap();
        self.current_sink.append(source)
    }

    pub fn pause(&mut self) {
        self.pause_time = Some(time_ms_now());
        self.current_sink.pause();
    }

    pub fn play(&mut self) {
        let interval = time_ms_now() - self.pause_time.unwrap();
        self.time_of_start = Some(self.time_of_start.unwrap() + interval);
        self.pause_time = None;
        self.current_sink.play();
    }

    fn clear(&mut self) {
        self.current_sink.stop();
        self.is_empty = true;
        self.current_sink = Arc::new(rodio::Sink::try_new(&self.handle.1).unwrap());
    }

    pub fn get_time(&self) -> u64 {
        match self.pause_time {
            None => match self.time_of_start {
                None => 0,
                Some(time_of_start) => get_interval_secs(time_of_start, time_ms_now()),
            },
            Some(pause_time) => match self.time_of_start {
                None => 0,
                Some(time_of_start) => get_interval_secs(time_of_start, pause_time),
            },
        }
    }

    pub fn get_current_track(&self) -> Option<&TrackEntity> {
        match &self.current_track {
            None => None,
            Some(track) => Some(track),
        }
    }

    pub async fn play_external_track(&mut self, track: TrackEntity) {
        self.current_track = Some(track.clone());
        self.time_of_start = Some(time_ms_now());
        self.pause_time = None;
        self.clear();
        self.append_external_track(&track).await;
        let sink = self.current_sink.clone();
        self.is_empty = false;
        thread::spawn(move || {
            sink.sleep_until_end();
        });
    }

    pub async fn play_track(&mut self, track: TrackEntity) {
        if track.get_path().split("://").count() > 1 {
            return self.play_external_track(track).await;
        }
        self.current_track = Some(track.clone());
        self.time_of_start = Some(time_ms_now());
        self.pause_time = None;
        self.clear();
        self.append_track(&track);
        let sink = self.current_sink.clone();
        self.is_empty = false;
        thread::spawn(move || {
            sink.sleep_until_end();
        });
    }
}
