use crate::app::modules::playlist::playlist_repository::PlaylistRepository;
use crate::app::modules::track::track_entity::TrackEntity;

pub struct Playlist {
    repository: PlaylistRepository,
}

impl Playlist {
    pub fn new(repository: PlaylistRepository) -> Self {
        Self { repository }
    }

    pub fn get_track_list(&self) -> Vec<TrackEntity> {
        self.repository.get_local_playlist()
    }
}
