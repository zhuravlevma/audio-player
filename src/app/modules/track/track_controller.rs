use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::ctx::Ctx;
use crate::app::modules::track::external_track_view::ExternalTrackView;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::modules::track::track_service::TrackService;
use crate::app::modules::track::track_view::TrackView;
use crate::app::routing::Commands;
use crate::infra::next::Next;

pub struct TrackController {
    track_service: TrackService,
}

impl TrackController {
    pub fn new(track_service: TrackService) -> Self {
        Self { track_service }
    }

    pub fn get_playing_track(&self, _request: Next, ctx: &mut Ctx) -> Next {
        self.track_service
            .get_current_track(ctx.get_player_entity())
    }

    pub async fn play_track(&self, ctx: &mut Ctx, track: TrackEntity) -> Next {
        self.track_service
            .play_track(ctx.get_player_entity_mut(), &track)
            .await
    }

    pub fn pause(&self, _request: Next, ctx: &mut Ctx) -> Next {
        ctx.pause_current_track();
        Next::new(Commands::Playlist(PlaylistCommand::GetPlayingTrack))
    }

    pub fn track_continue(&self, _request: Next, ctx: &mut Ctx) -> Next {
        ctx.continue_current_track();
        Next::new(Commands::Playlist(PlaylistCommand::GetPlayingTrack))
    }
}
