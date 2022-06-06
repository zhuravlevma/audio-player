use crate::app::command::home_command::HomeCommand;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::menu::Menu;
use crate::utils::view::View;
use terminal_menu::{button, label};

pub struct HomeView {}

impl HomeView {
    pub fn get_menu_with_header(track_path: &str, time: u64) -> Next {
        let items = vec![
            label(format!("Track {}  {} s", track_path, time)),
            label("Menu"),
            button("Local Playlist"),
            button("New Playlist"),
            button("Exit"),
        ];
        HomeView::response(Menu::create_and_handle(items))
    }

    pub fn get_menu_without_header() -> Next {
        let items = vec![
            label("Menu"),
            button("Local Playlist"),
            button("New Playlist"),
            button("Exit"),
        ];
        HomeView::response(Menu::create_and_handle(items))
    }
}

impl View for HomeView {
    fn response(command_str: String) -> Next {
        match command_str.as_ref() {
            "Local Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetLocalPlaylist), None),
            "New Playlist" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist), None),
            "Exit" => Next::new(Commands::MainMenu(HomeCommand::Exit), None),
            _ => Next::new(Commands::NotFound, None),
        }
    }
}
