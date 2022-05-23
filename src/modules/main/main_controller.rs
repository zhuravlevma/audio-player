use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::views::menu_view::MenuView;

pub struct MainController {}

impl MainController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn exit(&self) -> Next {
        std::process::exit(0)
    }

    pub fn show_menu(&self) -> Next {
        let s = MenuView::get(&String::from(""), 0);
        match s.as_ref() {
            "Exit" => Next::new(Commands::Exit, None),
            "TrackList" => Next::new(Commands::GetPlaylist, None),
            _ => Next::new(Commands::Exit, None),
        }
    }
}
