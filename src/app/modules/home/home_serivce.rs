use crate::app::modules::home::home_view::HomeView;
use crate::infra::next::Next;

pub struct HomeService {
    home_view: HomeView,
}

impl HomeService {
    pub fn new(home_view: HomeView) -> Self {
        Self { home_view }
    }
    pub fn exit() -> Next {
        std::process::exit(0)
    }
}
