use crate::app::modules::track::track_entity::TrackEntity;

#[derive(Clone)]
pub enum PlaylistCommand {
    GetPlayingTrack,
    Input(TrackEntity),
}
