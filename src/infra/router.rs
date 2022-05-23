use crate::app::routing::Routing;

pub struct Router {
    routing: Routing,
}

use crate::infra::next::Next;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RouterError {
    #[error("Your `{0}` is incorrect")]
    Invalid(String),
}

impl Router {
    pub fn new() -> Self {
        Self {
            routing: Routing::new(),
        }
    }

    pub fn run(&mut self, route_start: Next) -> Result<(), RouterError> {
        let mut point = route_start;

        loop {
            let point_clone = point.clone();
            let result = self.routing.routes(point_clone);
            point = result;
        }
    }
}
