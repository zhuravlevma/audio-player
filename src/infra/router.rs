use crate::app::routing::Routing;
use crate::infra::route::Route;

pub struct Router {
    routing: Routing,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routing: Routing::new(),
        }
    }

    pub fn run(&mut self, route_start: Route) {
        let mut point = route_start;

        loop {
            let path = point.route_path.clone() + "/" + &point.command;

            let result = self.routing.routes(&path, point.clone());
        }
    }
}
