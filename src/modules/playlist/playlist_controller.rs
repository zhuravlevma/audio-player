use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::infra::request::Request;
use crate::modules::playlist::playlist_entity::Playlist;
use crate::views::playlist_view::PlaylistView;
use std::collections::HashMap;
use crate::app::ctx::Ctx;

pub struct PlaylistController {
    playlist_service: Playlist,
}

impl PlaylistController {
    pub fn new(playlist_service: Playlist) -> Self {
        Self { playlist_service }
    }
    pub fn get_track_list(&self, _route_data: Next, _ctx: &mut Ctx) -> Next {
        let tracks = self.playlist_service.get_track_listv2();
        match self.playlist_service.get_current_track() {
            None => {
                let result = PlaylistView::getv2("", 0, tracks);
                match result.as_ref() {
                    "Back" => Next::new(Commands::BackToMain, None),
                    _ => Next::new(
                        Commands::PlayTrack,
                        Some(Request::new(HashMap::from([("track".to_string(), result)]))),
                    ),
                }
            }
            Some(track) => {
                let result = PlaylistView::getv2(track.get_path(), track.get_start(), tracks);

                match result.as_ref() {
                    "Back" => Next::new(Commands::BackToMain, None),
                    _ => Next::new(
                        Commands::PlayTrack,
                        Some(Request::new(HashMap::from([("track".to_string(), result)]))),
                    ),
                }
            }
        }
    }
    pub fn back(&self, _request: Next, _ctx: &mut Ctx) -> Next {
        Next::new(Commands::GetMainMenu, None)
    }
}
