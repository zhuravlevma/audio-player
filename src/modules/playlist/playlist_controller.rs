use crate::infra::next::Next;
use crate::infra::request::Request;
use crate::infra::route::Route;
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
    pub fn get_track_list(&self, _route_data: Next) -> Next {
        match self.playlist_service.get_current_track() {
            None => {
                let tracks = self.playlist_service.get_track_listv2();
                let result = PlaylistView::getv2("", 0, tracks);
                if result.eq("Back") {
                    return Next::new(Route::new("playlist", "Back"), None);
                }
                Next::new(
                    Route::new("track", "play"),
                    Some(Request::new(HashMap::from([("track".to_string(), result)]))),
                )
            }
            Some(track) => {
                let tracks = self.playlist_service.get_track_listv2();
                let result = PlaylistView::getv2(track.get_path(), track.get_start(), tracks);
                if result.eq("Back") {
                    return Next::new(Route::new("playlist", "Back"), None);
                }
                Next::new(
                    Route::new("track", "play"),
                    Some(Request::new(HashMap::from([("track".to_string(), result)]))),
                )
            }
        }
    }
    pub fn back(&self) -> Next {
        Next::new(Route::new("main", "Show"), None)
    }
}
