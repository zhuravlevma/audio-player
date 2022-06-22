use crate::app::ctx::Ctx;
use crate::app::modules::home::home_serivce::HomeService;
use crate::infra::next::Next;

pub struct HomeController {
    home_service: HomeService,
}

impl HomeController {
    pub fn new(home_service: HomeService) -> Self {
        Self { home_service }
    }

    pub fn exit(&self, _request: Next, _ctx: &Ctx) -> Next {
        self.home_service.exit()
    }

    pub fn show_menu(&self, _request: Next, ctx: &mut Ctx) -> Next {
        self.home_service.get_menu(ctx.get_player_entity())
    }
}
