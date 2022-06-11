use crate::app::modules::track::track_entity::TrackEntity;

#[derive(Clone)]
pub enum TrackCommand {
    Pause,
    Continue,
    PlayTrack(TrackEntity),
}
