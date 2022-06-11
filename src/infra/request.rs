use crate::app::modules::track::track_entity::TrackEntity;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Request {
    pub body: HashMap<String, String>,
    pub track: Option<TrackEntity>,
}

impl Request {
    pub fn new(body: HashMap<String, String>) -> Self {
        Self { body, track: None }
    }

    pub fn set_track(&mut self, track: TrackEntity) {
        self.track = Some(track);
    }
}
