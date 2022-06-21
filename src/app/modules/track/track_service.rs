use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::ctx::player::player_entity::Player;
use crate::app::modules::track::external_track_view::ExternalTrackView;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::modules::track::track_view;
use crate::app::modules::track::track_view::TrackView;
use crate::app::routing::Commands;
use crate::infra::next::Next;

pub struct TrackService {
    track_view: TrackView,
    external_track_view: ExternalTrackView,
}

impl TrackService {
    pub fn new(track_view: TrackView, external_track_view: ExternalTrackView) -> Self {
        Self {
            track_view,
            external_track_view,
        }
    }

    pub fn get_current_track(&self, player: &Player) -> Next {
        match player.get_current_track() {
            None => self.track_view.not_found(),
            Some(track) => match track.is_external {
                true => match player.pause_time {
                    None => self.external_track_view.get_track_with_header(
                        track.get_name(),
                        player.get_time(),
                    ),
                    Some(_) => {
                        self.external_track_view.get_pause_track(track.get_name(), player.get_time())
                    }
                },
                false => match player.pause_time {
                    None => self.track_view.get_track_with_header(track.get_name(), player.get_time()),
                    Some(_) => self.track_view.get_pause_track(track.get_name(), player.get_time()),
                },
            },
        }
    }

    pub async fn play_track(&self, player: &mut Player, track: &TrackEntity) -> Next {
        player.play_track(track.clone()).await;
        Next::new(Commands::Playlist(PlaylistCommand::GetPlayingTrack))
    }
}
