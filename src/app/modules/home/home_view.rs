use crate::app::command::home_command::HomeCommand;
use crate::app::ctx::player::player_entity::Player;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::menu::Menu;
use crossterm::style::Color;
use terminal_menu::*;

pub struct HomeView {}

impl HomeView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_menu(&self, player: &Player) -> Next {
        match player.get_current_track() {
            None => self.get_menu_without_header(),
            Some(track) => self.get_menu_with_header(track.get_name(), player.get_time()),
        }
    }

    pub fn get_menu_with_header(&self, track_name: &str, time: u64) -> Next {
        let items = vec![
            label(format!("♬ {} ⧗ {} s", track_name, time)).colorize(Color::Magenta),
            button("Local Playlist"),
            button("New Playlist"),
            button("Popular Playlist"),
            button("Exit"),
        ];
        match Menu::create_and_handle(items).as_ref() {
            "Local Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetLocalPlaylist)),
            "New Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
            "Popular Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetPopularPlaylist)),
            "Exit" => Next::new(Commands::MainMenu(HomeCommand::Exit)),
            _ => Next::new(Commands::NotFound),
        }
    }

    pub fn get_menu_without_header(&self) -> Next {
        let items = vec![
            button("Local Playlist"),
            button("New Playlist"),
            button("Popular Playlist"),
            button("Exit"),
        ];
        match Menu::create_and_handle(items).as_ref() {
            "Local Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetLocalPlaylist)),
            "New Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
            "Popular Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetPopularPlaylist)),
            "Exit" => Next::new(Commands::MainMenu(HomeCommand::Exit)),
            _ => Next::new(Commands::NotFound),
        }
    }
}
