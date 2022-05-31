use crate::modules::playlist::playlist_repository::PlaylistRepository;
use crate::modules::track::track_entity::TrackEntity;

pub struct Playlist {
    pub current_track: Option<TrackEntity>,
    repository: PlaylistRepository,
}

impl Playlist {
    pub fn new(repository: PlaylistRepository) -> Self {
        Self {
            current_track: Option::None,
            repository
        }
    }

    pub fn get_track_list(&self) -> Vec<TrackEntity> {
        self.repository.get_local_playlist()
    }

    pub fn get_current_track(&self) -> &Option<TrackEntity> {
        &self.current_track
    }
}
