use crate::app::ctx::player::player_entity::Player;
use crate::app::modules::home::home_view::HomeView;
use crate::infra::next::Next;

pub struct HomeService {
    home_view: HomeView,
}

impl HomeService {
    pub fn new(home_view: HomeView) -> Self {
        Self { home_view }
    }
    pub fn exit(&self) -> Next {
        std::process::exit(0)
    }

    pub fn get_menu(&self, player: &Player) -> Next {
        self.home_view.get_menu(player)
    }
}
