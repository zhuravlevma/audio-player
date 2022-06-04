use crate::app::ctx::Ctx;
use crate::app::modules::main::main_view::MainMenuEvents;
use crate::app::modules::playlist::playlist_entity::Playlist;
use crate::app::modules::playlist::playlist_view::{PlaylistEvents, PlaylistView};
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::modules::track::track_view::TrackEvents;
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

    fn response(&self, ctx: &Ctx, tracks: Vec<TrackEntity>) -> Next {
        let response = match ctx.player.get_current_track() {
            None => PlaylistView::get_playlist_without_header(&tracks),
            Some(track) => PlaylistView::get_playlist_with_header(
                &tracks,
                track.get_path(),
                ctx.player.get_time(),
            ),
        };

        match response.as_ref() {
            "Back" => Next::new(Commands::Playlist(PlaylistEvents::Back), None),
            _ => {
                if let Some(track) = ctx.player.get_current_track() {
                    if track.get_path().eq(&response) {
                        return Next::new(Commands::Playlist(PlaylistEvents::InputTrack), None);
                    }
                }

                Next::new(
                    Commands::Track(TrackEvents::PlayTrack),
                    Some(Request::new(HashMap::from([
                        ("track".to_string(), response),
                        ("is_external".to_string(), "false".to_string()),
                    ]))),
                )
            }
        }
    }

    pub fn back(&self, _request: Next, _ctx: &mut Ctx) -> Next {
        Next::new(Commands::MainMenu(MainMenuEvents::GetMenu), None)
    }
}
