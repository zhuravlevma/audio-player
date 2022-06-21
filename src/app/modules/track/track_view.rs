use crate::app::command::home_command::HomeCommand;
use crate::app::command::track_command::TrackCommand;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::menu_factory::MenuFactory;
use crossterm::style::Color;
use terminal_menu::{back_button, button, label};

pub struct TrackView {
    menu_factory: MenuFactory,
}

impl TrackView {
    pub fn new(menu_factory: MenuFactory) -> Self {
        Self { menu_factory }
    }
    pub fn get_track_with_header(&self, track_name: &str, time: u64) -> Next {
        let items = vec![
            label(format!("♬ {} ⧗ {} s", track_name, time)).colorize(Color::Magenta),
            button("Pause"),
            back_button("Back"),
        ];

        match MenuFactory::create_and_handle(items).as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetLocalPlaylist)),
            "Pause" => Next::new(Commands::Track(TrackCommand::Pause)),
            _ => Next::new(Commands::NotFound),
        }
    }

    pub fn get_pause_track(&self, track_name: &str, time: u64) -> Next {
        let items = vec![
            label(format!("♬ {} ⧗ {} s", track_name, time)).colorize(Color::Magenta),
            button("Continue"),
            button("Back"),
        ];

        match MenuFactory::create_and_handle(items).as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetLocalPlaylist)),
            "Continue" => Next::new(Commands::Track(TrackCommand::Continue)),
            _ => Next::new(Commands::NotFound),
        }
    }

    pub fn not_found(&self) -> Next {
        Next::new(Commands::NotFound)
    }
}
