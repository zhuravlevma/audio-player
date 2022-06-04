use terminal_menu::{button, label};
use crate::utils::menu::Menu;

pub struct TrackView {}

#[derive(Clone)]
pub enum TrackEvents {
    Pause,
    Continue,
    Back,
    PlayTrack,
}

impl TrackView {
    pub fn get_track_with_header(track_path: &str, time: u64) -> String {
        let items = vec![
            label(format!("Track {}  {} s", track_path, time)),
            button("Pause"),
            button("Back"),
        ];
        Menu::create_and_handle(items)
    }

    pub fn get_pause_track(track_path: &str, time: u64) -> String {
        let items = vec![
            label(format!("Track {}  {} s", track_path, time)),
            button("Continue"),
            button("Back"),
        ];
        Menu::create_and_handle(items)
    }

    pub fn not_found() -> String {
        let items= vec![label("error"), button("Back")];
        Menu::create_and_handle(items)
    }
}
