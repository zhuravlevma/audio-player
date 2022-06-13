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
        PlaylistView::get_playlist(ctx.get_player_entity(), &tracks)
    }

    pub async fn get_new_playlist(
        &mut self,
        _request: Next,
        ctx: &mut Ctx,
    ) -> Result<Next, Box<dyn Error>> {
        let tracks = self.playlist_service.get_new_playlist().await?;
        Ok(PlaylistView::get_playlist(ctx.get_player_entity(), &tracks))
    }

    pub fn input(&self, ctx: &mut Ctx, track: TrackEntity) -> Next {
        if let Some(current_track) = ctx.get_player_entity().get_current_track() {
            if current_track.path_is_equal(track.get_path()) {
                return Next::new(Commands::Playlist(PlaylistCommand::GetPlayingTrack));
            }
        }
        Next::new(Commands::Track(TrackCommand::PlayTrack(track)))
    }
}
