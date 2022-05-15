use crate::infra::route::Route;
use crate::infra::router::Router;
use crate::modules::main::main_controller::MainController;
use crate::modules::player::player_entity::Player;
use crate::modules::playlist::playlist_controller::PlaylistController;
use crate::modules::playlist::playlist_entity::Playlist;
use crate::modules::track::track_controller::TrackController;

pub struct Routing {
    playlist_controller: PlaylistController,
    track_controller: TrackController,
    main_controller: MainController,
    player: Player,
}

impl Routing {
    pub fn new() -> Self {
        Self {
            playlist_controller: PlaylistController::new(Playlist::new("./assets")),
            track_controller: TrackController::new(),
            main_controller: MainController::new(),
            player: Player::new(),
        }
    }

    pub fn routes(&mut self, path: &str, route: Route) -> Route {
        match path {
            "main/Show" => self.main_controller.show_menu(),
            "main/TrackList" => self.main_controller.playlist(),
            "main/Exit" => self.main_controller.exit(),
            "playlist/TrackList" => self.playlist_controller.get_track_list(route),
            "playlist/Back" => self.playlist_controller.back(),
            _ => Route::new("", ""),
        }
    }
}
