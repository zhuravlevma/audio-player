use crate::domains::track_entity::TrackEntity;
use terminal_menu::{button, label, menu, TerminalMenu, TerminalMenuItem};

pub struct PlaylistView {}

impl PlaylistView {
    pub fn get(current_track: &str, s: u64, track_list: &[TrackEntity]) -> TerminalMenu {
        if s != 0 {
            let mut items: Vec<TerminalMenuItem> =
                vec![label(format!("Track {}  {} s", current_track, s))];
            track_list
                .iter()
                .for_each(|el| items.push(button(el.track_path.clone())));
            items.push(button("Back"));
            return menu(items);
        }
        let mut items: Vec<TerminalMenuItem> = track_list
            .iter()
            .map(|el| button(el.track_path.clone()))
            .collect();
        items.push(button("Back"));
        menu(items)
    }
}
