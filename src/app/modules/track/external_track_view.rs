use crate::app::command::home_command::HomeCommand;
use crate::app::command::track_command::TrackCommand;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::menu_factory::MenuFactory;
use crossterm::style::Color;
use std::collections::HashMap;
use terminal_menu::{button, label};
use crate::utils::colors::Colors;

pub struct ExternalTrackView {
    menu_factory: MenuFactory,
}

impl ExternalTrackView {
    pub fn new(menu_factory: MenuFactory) -> Self {
        Self { menu_factory }
    }
    pub fn get_track_with_header(&self, track_name: &str, time: u64) -> Next {
        let items = vec![
            label(format!("♬ {} ⧗ {} s", track_name, time)).colorize(Color::Magenta),
            button("Pause"),
            button("Back"),
            // self.menu_factory.set_color(self.menu_factory.label(&format!("♬ {} ⧗ {} s", track_name, time)), Colors::Label)
        ];

        match MenuFactory::create_and_handle(items).as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
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
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
            "Continue" => Next::new(Commands::Track(TrackCommand::Continue)),
            _ => Next::new(Commands::NotFound),
        }
    }
}
