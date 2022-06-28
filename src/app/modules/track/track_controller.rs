use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::command::track_command::TrackCommand;
use crate::app::ctx::Ctx;
use crate::app::modules::track::external_track_view::ExternalTrackView;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::modules::track::track_service::TrackService;
use crate::app::modules::track::track_view::TrackView;
use crate::app::routing::Commands;
use crate::infra::next::Next;

pub struct TrackController {
    track_service: TrackService,
    track_view: TrackView,
    external_track_view: ExternalTrackView,
}

impl TrackController {
    pub fn new(
        track_service: TrackService,
        track_view: TrackView,
        external_track_view: ExternalTrackView,
    ) -> Self {
        Self {
            track_service,
            track_view,
            external_track_view,
        }
    }

    pub fn get_playing_track(&self, _request: Next, ctx: &mut Ctx) -> Next {
        let time = ctx.get_player_entity().get_time();
        let player = ctx.get_player_entity();
        match player.get_current_track() {
            None => self.track_view.not_found(),
            Some(track) => match track.is_external {
                true => match player.pause_time {
                    None => self
                        .external_track_view
                        .get_track_with_header(track.get_name(), time),
                    Some(_) => self
                        .external_track_view
                        .get_pause_track(track.get_name(), time),
                },
                false => match player.pause_time {
                    None => self
                        .track_view
                        .get_track_with_header(track.get_name(), time),
                    Some(_) => self.track_view.get_pause_track(track.get_name(), time),
                },
            },
        }
    }

    pub async fn play_track(&self, ctx: &mut Ctx, track: TrackEntity) -> Next {
        self.track_service
            .play_track(ctx.get_player_entity_mut(), &track)
            .await;
        Next::new(Commands::Playlist(PlaylistCommand::GetPlayingTrack))
    }

    pub async fn download(&self, _request: Next, ctx: &mut Ctx) -> Next {
        self.track_service
            .download_track(ctx.get_player_entity().get_current_track().unwrap())
            .await;
        Next::new(Commands::Track(TrackCommand::Refresh))
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
