use crate::app::routing::Routing;
use crate::infra::route::Route;
use crate::infra::router::{Router, RouterError};
use crate::modules::main::main_controller::MainController;
use crate::modules::player::player_entity::Player;
use crate::modules::playlist::playlist_controller::PlaylistController;
use crate::modules::playlist::playlist_entity::Playlist;
use crate::modules::track::track_controller::TrackController;
use crate::utils::console::ConsoleError;
use thiserror::Error;
use crate::infra::next::Next;

#[derive(Error, Debug)]
pub enum RunError {
    #[error("io error")]
    IoError(#[from] ConsoleError),
    #[error("route error")]
    RouteError(#[from] RouterError),
}

pub struct Run {
    playlist_controller: PlaylistController,
    track_controller: TrackController,
    main_controller: MainController,
    player: Player,
}

impl Run {
    pub fn new() -> Self {
        Self {
            playlist_controller: PlaylistController::new(Playlist::new("./assets")),
            track_controller: TrackController::new(),
            main_controller: MainController::new(),
            player: Player::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), RunError> {
        let mut router = Router::new();
        router.run(Next::new(Route::new("playlist", "TrackList"), None));
        Ok(())
        // let mut point = Route::new("playlist", "TrackList");
        // loop {
        //     point = match point.route_path.as_ref() {
        //         "main" => match point.command.as_ref() {
        //             "Show" => self.main_controller.show_menu(),
        //             "TrackList" => self.main_controller.playlist(),
        //             "Exit" => self.main_controller.exit(),
        //             _ => self.main_controller.error(),
        //         },
        //         "playlist" => match point.command.as_ref() {
        //             "Back" => self.playlist_controller.back(),
        //             "TrackList" => self.playlist_controller.get_track_list(point),
        //             _ => self.playlist_controller.play_track(point, &mut self.player),
        //         },
        //         "track" => match point.command.as_ref() {
        //             "Show" => self.track_controller.get_current_track(&self.player),
        //             "Back" => self.track_controller.back(),
        //             _ => self.track_controller.error(),
        //         },
        //         _ => Route::new("error", "error"),
        //     }
        // }
    }
}
