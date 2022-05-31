#[derive(Clone, Debug)]
pub struct TrackEntity {
    track_path: String,
    is_external: bool,
}

impl TrackEntity {
    pub fn new(track_path: String, is_external: bool) -> Self {
        Self {
            track_path,
            is_external,
        }
    }
    pub fn get_path(&self) -> &String {
        &self.track_path
    }
}
