use crate::modules::track::track_entity::TrackEntity;
use terminal_menu::{button, label, menu, mut_menu, run, TerminalMenuItem};

pub struct PlaylistView {}

impl PlaylistView {
    pub fn getv2(current_track: &str, s: u64, track_list: &[TrackEntity]) -> String {
        if s != 0 {
            let mut items: Vec<TerminalMenuItem> =
                vec![label(format!("Track {}  {} s", current_track, s))];
            track_list
                .iter()
                .for_each(|el| items.push(button(el.get_path().to_string())));
            items.push(button("Back"));
            let t = menu(items);
            run(&t);
            let s = mut_menu(&t).selected_item_name().to_string();
            return s;
        }
        let mut items: Vec<TerminalMenuItem> = track_list
            .iter()
            .map(|el| button(el.get_path().to_string()))
            .collect();
        items.push(button("Back"));
        let t = menu(items);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }
}
