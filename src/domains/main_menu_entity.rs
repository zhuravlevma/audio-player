use crate::app::time::{get_interval_secs, time_ms_now};
use std::time::Duration;
use terminal_menu::{mut_menu, run, TerminalMenu};
use crate::views::menu_view::MenuView;

#[derive(Clone, Debug)]
pub struct MainMenuEntity {}

impl MainMenuEntity {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_menu(&self) -> TerminalMenu {
        MenuView::get(&String::from(""), 0)
    }

    pub fn run(&self, terminal_menu: TerminalMenu) -> String {
        run(&terminal_menu);
        format!("main/|/{}", mut_menu(&terminal_menu).selected_item_name().to_string())
    }
}
