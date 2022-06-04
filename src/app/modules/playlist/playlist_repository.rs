use crate::app::modules::external::muzati::Muzati;
use crate::app::modules::track::track_entity::TrackEntity;
use std::error::Error;
use std::fs;

pub struct PlaylistRepository {
    local_path: String,
    external_api: Muzati,
}

impl PlaylistRepository {
    pub fn new(local_path: &str, external_api: Muzati) -> Self {
        Self {
            local_path: local_path.to_string(),
            external_api,
        }
    }

    pub fn get_local_playlist(&self) -> Vec<TrackEntity> {
        fs::read_dir(&self.local_path)
            .unwrap()
            .into_iter()
            .map(|path| TrackEntity::new(path.unwrap().path().display().to_string(), false))
            .collect()
    }

    pub async fn get_new_tracks(&self) -> Result<Vec<TrackEntity>, Box<dyn Error>> {
        let resp = self.external_api.get_new_tracks().await?;
        let tracks: Vec<TrackEntity> = resp
            .iter()
            .map(|el| TrackEntity::new(el.track_url.to_string(), true))
            .collect();
        Ok(tracks)
    }
}
