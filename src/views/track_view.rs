use terminal_menu::{button, label, menu, TerminalMenu};

pub struct TrackView {}

impl TrackView {
    pub fn get(path: String) -> TerminalMenu {
        menu(vec![
            label(format!("Track {}", path)),
            button("Pause"),
            button("Back"),
        ])
    }
}
