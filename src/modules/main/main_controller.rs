use crate::infra::next::Next;
use crate::infra::route::Route;
use crate::views::menu_view::MenuView;

pub struct MainController {}

impl MainController {
    pub fn new() -> Self {
        Self {}
    }
    pub fn exit(&self) -> Next {
        std::process::exit(0)
    }

    pub fn error(&self) -> Next {
        Next::new(Route::new("main", "error"), None)
    }

    pub fn show_menu(&self) -> Next {
        let s = MenuView::get(&String::from(""), 0);
        Next::new(Route::new("main", s), None)
    }

    pub fn playlist(&self) -> Next {
        Next::new(Route::new("playlist", "TrackList"), None)
    }
}
