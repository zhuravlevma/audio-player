use terminal_menu::{menu, mut_menu, run, TerminalMenuItem};

pub struct Menu {}

impl Menu {
    pub fn create_and_handle(items: Vec<TerminalMenuItem>) -> String {
        let terminal_menu = menu(items);
        run(&terminal_menu);
        let response = mut_menu(&terminal_menu).selected_item_name().to_string();
        response
    }
}
