use crate::app::modules::track::track_entity::TrackEntity;
use terminal_menu::{button, label, TerminalMenuItem};
use crate::utils::menu::Menu;

#[derive(Clone)]
pub enum PlaylistEvents {
    InputTrack,
    Back,
}

pub struct PlaylistView {}

impl PlaylistView {
    pub fn get_playlist_with_header(
        track_list: &[TrackEntity],
        track_path: &str,
        time: u64,
    ) -> String {
        let mut items: Vec<TerminalMenuItem> =
            vec![label(format!("Track {}  {} s", track_path, time))];
        track_list
            .iter()
            .for_each(|el| items.push(button(el.get_path().to_string())));
        items.push(button("Back"));
        Menu::create_and_handle(items)
    }

    pub fn get_playlist_without_header(track_list: &[TrackEntity]) -> String {
        let mut items: Vec<TerminalMenuItem> = track_list
            .iter()
            .map(|el| button(el.get_path().to_string()))
            .collect();
        items.push(button("Back"));
        Menu::create_and_handle(items)
    }
}
