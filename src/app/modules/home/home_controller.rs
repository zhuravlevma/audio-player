use crate::app::ctx::Ctx;
use crate::app::modules::home::home_serivce::HomeService;
use crate::app::modules::home::home_view::HomeView;
use crate::infra::next::Next;

pub struct HomeController {
    home_service: HomeService,
    home_view: HomeView,
}

impl HomeController {
    pub fn new(home_service: HomeService, home_view: HomeView) -> Self {
        Self {
            home_service,
            home_view,
        }
    }

    pub fn exit(&self, _request: Next, _ctx: &Ctx) -> Next {
        std::process::exit(0);
    }

    pub fn show_menu(&self, _request: Next, ctx: &mut Ctx) -> Next {
        self.home_view.get_menu(ctx.get_player_entity())
    }
}
