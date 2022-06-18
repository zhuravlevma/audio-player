use crate::app::command::home_command::HomeCommand;
use crate::app::ctx::player::player_entity::Player;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::menu::Menu;
use terminal_menu::{button, label};

pub struct HomeView {}

impl HomeView {
    pub fn get_menu(player: &Player) -> Next {
        match player.get_current_track() {
            None => HomeView::get_menu_without_header(),
            Some(track) => HomeView::get_menu_with_header(track.get_name(), player.get_time()),
        }
    }

    pub fn get_menu_with_header(track_name: &str, time: u64) -> Next {
        let items = vec![
            label(format!("Track {}  {} s", track_name, time)),
            label("Menu"),
            button("Local Playlist"),
            button("New Playlist"),
            button("Exit"),
        ];
        match Menu::create_and_handle(items).as_ref() {
            "Local Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetLocalPlaylist)),
            "New Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
            "Exit" => Next::new(Commands::MainMenu(HomeCommand::Exit)),
            _ => Next::new(Commands::NotFound),
        }
    }

    pub fn get_menu_without_header() -> Next {
        let items = vec![
            label("Menu"),
            button("Local Playlist"),
            button("New Playlist"),
            button("Exit"),
        ];
        match Menu::create_and_handle(items).as_ref() {
            "Local Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetLocalPlaylist)),
            "New Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
            "Exit" => Next::new(Commands::MainMenu(HomeCommand::Exit)),
            _ => Next::new(Commands::NotFound),
        }
    }
}
