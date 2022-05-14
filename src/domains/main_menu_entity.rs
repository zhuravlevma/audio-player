use crate::app::time::{get_interval_secs, time_ms_now};
use crate::views::menu_view::MenuView;
use std::time::Duration;
use terminal_menu::{mut_menu, run, TerminalMenu};
use crate::domains::route::Route;

#[derive(Clone, Debug)]
pub struct MainMenuEntity {}

impl MainMenuEntity {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_menu(&self) -> Route {
        let t = MenuView::get(&String::from(""), 0);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        Route::new("main", s)
    }
}
