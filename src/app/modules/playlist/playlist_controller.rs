use crate::app::command::home_command::HomeCommand;
use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::command::track_command::TrackCommand;
use crate::app::ctx::Ctx;
use crate::app::modules::playlist::playlist_service::Playlist;
use crate::app::modules::playlist::playlist_view::PlaylistView;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::infra::request::Request;
use std::collections::HashMap;
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
        &self,
        _request: Next,
        ctx: &mut Ctx,
    ) -> Result<Next, Box<dyn Error>> {
        let tracks = self.playlist_service.get_new_playlist().await?;
        Ok(self.response(ctx, tracks))
    }

    pub fn input(&self, request: Next, ctx: &mut Ctx) -> Next {
        let req = request.request.unwrap();
        let track_req = req.body.get("track").unwrap();
        if let Some(track) = ctx.player.get_current_track() {
            if track.get_path().eq(track_req) {
                return Next::new(Commands::Playlist(PlaylistCommand::InputTrack), None);
            }
        }
        Next::new(
            Commands::Track(TrackCommand::PlayTrack),
            Some(Request::new(HashMap::from([(
                "track".to_string(),
                track_req.to_string(),
            )]))),
        )
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

    pub fn back(&self, _request: Next, _ctx: &mut Ctx) -> Next {
        Next::new(Commands::MainMenu(HomeCommand::GetMenu), None)
    }
}
