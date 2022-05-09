use terminal_menu::{button, label, menu, TerminalMenu, TerminalMenuItem};

pub struct PlaylistView {}

impl PlaylistView {
    pub fn get(current_track: &str, s: u64, track_list: &[String]) -> TerminalMenu {
        if s != 0 {
            let mut items: Vec<TerminalMenuItem> =
                vec![label(format!("Track {}  {} s", current_track, s))];
            track_list.iter().for_each(|el| items.push(button(el)));
            items.push(button("Back"));
            return menu(items);
        }
        let mut items: Vec<TerminalMenuItem> = track_list.iter().map(button).collect();
        items.push(button("Back"));
        menu(items)
    }
}
