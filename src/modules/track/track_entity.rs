#[derive(Clone, Debug)]
pub struct TrackEntity {
    track_path: String,
}

impl TrackEntity {
    pub fn new(track_path: String) -> Self {
        Self { track_path }
    }
    pub fn get_path(&self) -> &String {
        &self.track_path
    }
}
