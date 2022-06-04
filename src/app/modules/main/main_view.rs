use terminal_menu::{button, label};
use crate::utils::menu::Menu;

pub struct MenuView {}

#[derive(Clone)]
pub enum MainMenuEvents {
    GetMenu,
    GetLocalPlaylist,
    GetExternalPlaylist,
    Exit,
}

impl MenuView {
    pub fn get_menu_with_header(track_path: &str, time: u64) -> String {
        let items = vec![
            label(format!("Track {}  {} s", track_path, time)),
            label("Menu"),
            button("TrackList"),
            button("Exit"),
        ];
        Menu::create_and_handle(items)
    }

    pub fn get_menu_without_header() -> String {
        let items = vec![label("Menu"), button("TrackList"), button("Exit")];
        Menu::create_and_handle(items)
    }
}
