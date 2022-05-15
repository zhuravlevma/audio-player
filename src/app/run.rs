use crate::infra::route::Route;
use crate::modules::main::main_controller::MainController;
use crate::modules::player::player_entity::Player;
use crate::modules::playlist::playlist_controller::PlaylistController;
use crate::modules::playlist::playlist_entity::Playlist;
use crate::modules::track::track_controller::TrackController;
use crate::utils::console::ConsoleError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunError {
    #[error("io error")]
    IoError(#[from] ConsoleError),
}

pub struct Run {
    player: Player,
}

impl Run {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), RunError> {
        let playlist_controller = PlaylistController::new(Playlist::new("./assets"));
        let mut point = Route::new("playlist", "TrackList");
        let track_controller = TrackController::new();
        let main_controller = MainController::new();
        loop {
            point = match point.route_path.as_ref() {
                "main" => match point.command.as_ref() {
                    "Show" => main_controller.show_menu(),
                    "TrackList" => main_controller.playlist(),
                    "Exit" => main_controller.exit(),
                    _ => main_controller.error(),
                },
                "playlist" => match point.command.as_ref() {
                    "Back" => playlist_controller.back(),
                    "TrackList" => playlist_controller.get_track_list(point),
                    _ => playlist_controller.play_track(point, &mut self.player),
                },
                "track" => match point.command.as_ref() {
                    "Show" => track_controller.get_current_track(&self.player),
                    "Back" => track_controller.back(),
                    _ => track_controller.error(),
                },
                _ => Route::new("error", "error"),
            }
        }
    }
}
