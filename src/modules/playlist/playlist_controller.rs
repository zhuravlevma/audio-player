use crate::app::ctx::Ctx;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::infra::request::Request;
use crate::modules::playlist::playlist_entity::Playlist;
use crate::views::playlist_view::PlaylistView;
use std::collections::HashMap;

pub struct PlaylistController {
    playlist_service: Playlist,
}

impl PlaylistController {
    pub fn new(playlist_service: Playlist) -> Self {
        Self { playlist_service }
    }

    pub fn get_track_list(&self, _route_data: Next, ctx: &Ctx) -> Next {
        let tracks = self.playlist_service.get_track_list();
        let response = match ctx.player.get_current_track() {
            None => PlaylistView::get_playlist_without_header(&tracks),
            Some(track) => PlaylistView::get_playlist_with_header(
                &tracks,
                track.get_path(),
                ctx.player.get_time(),
            ),
        };

        match response.as_ref() {
            "Back" => Next::new(Commands::BackToMain, None),
            _ => {
                if let Some(track) = ctx.player.get_current_track() {
                    if track.get_path().eq(&response) {
                        return Next::new(Commands::ShowTrack, None);
                    }
                }

                Next::new(
                    Commands::PlayTrack,
                    Some(Request::new(HashMap::from([
                        ("track".to_string(), response),
                        ("is_external".to_string(), "false".to_string()),
                    ]))),
                )
            }
        }
    }

    pub fn back(&self, _request: Next, _ctx: &mut Ctx) -> Next {
        Next::new(Commands::GetMainMenu, None)
    }
}
