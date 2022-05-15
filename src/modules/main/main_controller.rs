use crate::infra::route::Route;
use crate::views::menu_view::MenuView;

pub struct MainController {}

impl MainController {
    pub fn new() -> Self {
        Self {}
    }
    pub fn exit(&self) -> Route {
        std::process::exit(0)
    }

    pub fn error(&self) -> Route {
        Route::new("main", "error")
    }

    pub fn show_menu(&self) -> Route {
        let s = MenuView::get(&String::from(""), 0);
        Route::new("main", s)
    }

    pub fn playlist(&self) -> Route {
        Route::new("playlist", "TrackList")
    }
}
