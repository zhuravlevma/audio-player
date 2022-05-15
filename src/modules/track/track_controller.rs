use crate::infra::route::Route;
use crate::modules::player::player_entity::Player;
use crate::views::track_view::TrackView;

pub struct TrackController {}

impl TrackController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_current_track(&self, player: &Player) -> Route {
        match player.get_current_trackv2() {
            None => {
                let s = TrackView::not_found();
                Route::new("track", s)
            }
            Some(track) => {
                let s = TrackView::getv2(track.get_path());
                Route::new("track", s)
            }
        }
    }

    pub fn back(&self) -> Route {
        Route::new("playlist", "TrackList")
    }

    pub fn error(&self) -> Route {
        Route::new("track", "error")
    }
}
