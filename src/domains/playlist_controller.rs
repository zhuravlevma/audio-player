use crate::app::player::Player;
use crate::domains::playlist_entity::Playlist;
use crate::domains::route::Route;
use crate::domains::track_entity::TrackEntity;
use crate::views::playlist_view::PlaylistView;

pub struct PlaylistController {
    playlist_service: Playlist,
}

impl PlaylistController {
    pub fn new(playlist_service: Playlist) -> Self {
        Self {
            playlist_service
        }
    }
    pub fn get_track_list(&self, _route_data: Route) -> Route {
        match self.playlist_service.get_current_track() {
            None => {
                let tracks = self.playlist_service.get_track_listv2();
                let result = PlaylistView::getv2("", 0, tracks);
                Route::new(
                    "playlist",
                    result
                )
            }
            Some(track) => {
                let tracks = self.playlist_service.get_track_listv2();
                let result = PlaylistView::getv2(track.get_path(), track.get_start(), tracks);
                Route::new(
                    "playlist",
                    result
                )
            }
        }
    }

    pub fn play_track(&self, route_data: Route, player: &mut Player) -> Route {
        let track_name = route_data.command;
        let track = self.playlist_service.find_track(&track_name).unwrap();
        player.play_track(track);
        Route::new("track", "Show")
    }
}