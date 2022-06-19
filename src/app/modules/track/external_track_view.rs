use crate::app::command::home_command::HomeCommand;
use crate::app::command::track_command::TrackCommand;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::menu::Menu;
use crossterm::style::Color;
use std::collections::HashMap;
use terminal_menu::{button, label};

pub struct ExternalTrackView {}

impl ExternalTrackView {
    pub fn get_track_with_header(track_name: &str, time: u64) -> Next {
        let items = vec![
            label(format!("Track {}  {} s", track_name, time)).colorize(Color::Magenta),
            button("Pause"),
            button("Back"),
        ];

        match Menu::create_and_handle(items).as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
            "Pause" => Next::new(Commands::Track(TrackCommand::Pause)),
            _ => Next::new(Commands::NotFound),
        }
    }

    pub fn get_pause_track(track_name: &str, time: u64) -> Next {
        let items = vec![
            label(format!("♬ {} ⧗ {} s", track_name, time)).colorize(Color::Magenta),
            button("Continue"),
            button("Back"),
        ];
        match Menu::create_and_handle(items).as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
            "Continue" => Next::new(Commands::Track(TrackCommand::Continue)),
            _ => Next::new(Commands::NotFound),
        }
    }
}
