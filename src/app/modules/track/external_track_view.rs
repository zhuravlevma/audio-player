use crate::app::command::track_command::TrackCommand;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::menu::Menu;
use crate::utils::view::View;
use terminal_menu::{button, label};
use crate::app::command::home_command::HomeCommand;
use crate::app::command::playlist_command::PlaylistCommand;

pub struct ExternalTrackView {}

impl ExternalTrackView {
    pub fn get_track_with_header(track_path: &str, time: u64) -> Next {
        let items = vec![
            label(format!("Track {}  {} s", track_path, time)),
            button("Pause"),
            button("Back"),
        ];

        match Menu::create_and_handle(items).as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist), None),
            "Pause" => Next::new(Commands::Track(TrackCommand::Pause), None),
            _ => Next::new(Commands::NotFound, None),
        }
    }

    pub fn get_pause_track(track_path: &str, time: u64) -> Next {
        let items = vec![
            label(format!("Track {}  {} s", track_path, time)),
            button("Continue"),
            button("Back"),
        ];
        match Menu::create_and_handle(items).as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist), None),
            "Continue" => Next::new(Commands::Track(TrackCommand::Continue), None),
            _ => Next::new(Commands::NotFound, None),
        }
    }

    pub fn not_found() -> Next {
        Next::new(Commands::NotFound, None)
    }
}
