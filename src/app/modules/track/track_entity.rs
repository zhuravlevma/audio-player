#[derive(Clone, Debug)]
pub struct TrackEntity {
    track_path: String,
    track_name: String,
    artist: String,
    pub is_external: bool,
}

impl TrackEntity {
    pub fn new(track_path: String, track_name: String, artist: String, is_external: bool) -> Self {
        Self {
            track_path,
            is_external,
            track_name,
            artist,
        }
    }
    pub fn get_path(&self) -> &String {
        &self.track_path
    }

    pub fn get_name(&self) -> &String {
        &self.track_name
    }

    pub fn get_artist(&self) -> &String {
        &self.artist
    }

    pub fn path_is_equal(&self, path: &String) -> bool {
        self.track_path.eq(path)
    }
}
