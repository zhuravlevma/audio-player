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
        HomeView::get_menu(ctx.get_player_entity())
    }
}
