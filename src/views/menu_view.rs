use terminal_menu::{button, label, menu, TerminalMenu};

pub struct MenuView {}

impl MenuView {
    pub fn get(current_track: &str, s: u64) -> TerminalMenu {
        if s != 0 {
            return menu(vec![
                label(format!("Track {}  {} s", current_track, s)),
                label("Menu"),
                button("Track list"),
                button("Exit"),
            ]);
        }
        menu(vec![label("Menu"), button("Track list"), button("Exit")])
    }
}
