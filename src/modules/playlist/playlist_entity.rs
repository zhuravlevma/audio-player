use crate::modules::track::track_entity::TrackEntity;
use std::fs;

pub struct Playlist {
    pub current_track: Option<TrackEntity>,
    tracks: Vec<TrackEntity>,
}

impl Playlist {
    pub fn new(path: &str) -> Self {
        Self {
            current_track: Option::None,
            tracks: fs::read_dir(path)
                .unwrap()
                .into_iter()
                .map(|path| TrackEntity::new(path.unwrap().path().display().to_string()))
                .collect(),
        }
    }

    pub fn get_track_list(&self) -> &Vec<TrackEntity> {
        &self.tracks
    }

    pub fn get_current_track(&self) -> &Option<TrackEntity> {
        &self.current_track
    }

    pub fn find_track(&self, track_path: &str) -> Option<&TrackEntity> {
        self.tracks.iter().find(|&el| el.get_path() == track_path)
    }
}
