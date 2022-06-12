use crate::app::ctx::Ctx;
use crate::app::modules::home::home_view::HomeView;
use crate::infra::next::Next;

pub struct HomeController {}

impl HomeController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn exit(&self, _request: Next, _ctx: &Ctx) -> Next {
        std::process::exit(0)
    }

    pub fn show_menu(&self, _request: Next, ctx: &Ctx) -> Next {
        match ctx.get_player_entity().get_current_track() {
            None => HomeView::get_menu_without_header(),
            Some(track) => {
                HomeView::get_menu_with_header(track.get_path(), ctx.get_player_entity().get_time())
            }
        }
    }
}
