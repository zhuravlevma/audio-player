use crate::app::modules::playlist::playlist_repository::PlaylistRepository;
use crate::app::modules::track::track_entity::TrackEntity;
use std::error::Error;

pub struct Playlist {
    repository: PlaylistRepository,
}

impl Playlist {
    pub fn new(repository: PlaylistRepository) -> Self {
        Self { repository }
    }

    pub fn get_local_playlist(&self) -> Vec<TrackEntity> {
        self.repository.get_local_playlist()
    }

    pub async fn get_new_playlist(&mut self) -> Result<Vec<TrackEntity>, Box<dyn Error>> {
        self.repository.get_new_tracks().await
    }

    pub async fn get_popular_playlist(&mut self) -> Result<Vec<TrackEntity>, Box<dyn Error>> {
        self.repository.get_popular_tracks().await
    }
}
