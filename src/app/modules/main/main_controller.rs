use crate::app::ctx::Ctx;
use crate::app::modules::main::menu_view::{MainMenuEvents, MenuView};
use crate::app::routing::Commands;
use crate::infra::next::Next;

pub struct MainController {}

impl MainController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn exit(&self, _request: Next, _ctx: &Ctx) -> Next {
        std::process::exit(0)
    }

    pub fn show_menu(&self, _request: Next, ctx: &Ctx) -> Next {
        let response = match ctx.player.get_current_track() {
            None => MenuView::get_menu_without_header(),
            Some(track) => MenuView::get_menu_with_header(track.get_path(), ctx.player.get_time()),
        };

        match response.as_ref() {
            "Exit" => Next::new(Commands::MainMenu(MainMenuEvents::Exit), None),
            "TrackList" => Next::new(Commands::MainMenu(MainMenuEvents::GetLocalPlaylist), None),
            _ => Next::new(Commands::MainMenu(MainMenuEvents::Exit), None),
        }
    }
}
