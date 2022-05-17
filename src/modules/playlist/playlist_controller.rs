use crate::infra::next::Next;
use crate::infra::route::Route;
use crate::modules::player::player_entity::Player;
use crate::modules::playlist::playlist_entity::Playlist;
use crate::views::playlist_view::PlaylistView;

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
                Next::new(Route::new("playlist", result), None)
            }
            Some(track) => {
                let tracks = self.playlist_service.get_track_listv2();
                let result = PlaylistView::getv2(track.get_path(), track.get_start(), tracks);
                Next::new(Route::new("playlist", result), None)
            }
        }
    }

    pub fn play_track(&self, route_data: Next, player: &mut Player) -> Next {
        let track_name = route_data.route.command;
        let track = self.playlist_service.find_track(&track_name).unwrap();
        player.play_track(track);
        Next::new(Route::new("track", "Show"), None)
    }

    pub fn back(&self) -> Next {
        Next::new(Route::new("main", "Show"), None)
    }
}
