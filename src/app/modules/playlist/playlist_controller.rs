use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::command::track_command::TrackCommand;
use crate::app::ctx::Ctx;
use crate::app::modules::playlist::playlist_service::Playlist;
use crate::app::modules::playlist::playlist_view::PlaylistView;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use std::error::Error;

pub struct PlaylistController {
    playlist_service: Playlist,
}

impl PlaylistController {
    pub fn new(playlist_service: Playlist) -> Self {
        Self { playlist_service }
    }

    pub fn get_track_list(&self, _route_data: Next, ctx: &Ctx) -> Next {
        let tracks = self.playlist_service.get_local_playlist();
        self.response(ctx, tracks)
    }

    pub async fn get_new_playlist(
        &mut self,
        _request: Next,
        ctx: &mut Ctx,
    ) -> Result<Next, Box<dyn Error>> {
        let tracks = self.playlist_service.get_new_playlist().await?;
        Ok(self.response(ctx, tracks))
    }

    pub fn input(&self, ctx: &mut Ctx, track: TrackEntity) -> Next {
        let track_path = track.get_path().clone();
        if let Some(track_player) = ctx.player.get_current_track() {
            if track_player.get_path().eq(&track_path) {
                return Next::new(Commands::Playlist(PlaylistCommand::GetPlayingTrack));
            }
        }
        Next::new(Commands::Track(TrackCommand::PlayTrack(track)))
    }

    fn response(&self, ctx: &Ctx, tracks: Vec<TrackEntity>) -> Next {
        match ctx.player.get_current_track() {
            None => PlaylistView::get_playlist_without_header(&tracks),
            Some(track) => PlaylistView::get_playlist_with_header(
                &tracks,
                track.get_path(),
                ctx.player.get_time(),
            ),
        }
    }
}
