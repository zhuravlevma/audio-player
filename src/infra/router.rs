use crate::app::routing::Routing;

pub struct Router {
    routing: Routing,
}

use crate::infra::next::Next;
use thiserror::Error;
use crate::app::ctx::Ctx;

#[derive(Error, Debug)]
pub enum RouterError {
    #[error("Your `{0}` is incorrect")]
    Invalid(String),
}

impl Router {
    pub fn new(routing: Routing) -> Self {
        Self {
            routing,
        }
    }

    pub fn run(&mut self, route_start: Next, mut ctx: Ctx) -> Result<(), RouterError> {
        let mut point = route_start;
        loop {
            let point_clone = point.clone();
            let result = self.routing.routes(point_clone, &mut ctx);
            point = result;
        }
    }
}
