use terminal_menu::{button, label, menu, TerminalMenu};

pub struct ErrorView {}

impl ErrorView {
    pub fn get() -> TerminalMenu {
        menu(vec![label("Error"), button("Repeat")])
    }
}
