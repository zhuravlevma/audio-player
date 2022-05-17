use std::error::Error;
use crate::app::routing::Routing;
use crate::infra::route::Route;

pub struct Router {
    routing: Routing,
}

use thiserror::Error;
use crate::infra::next::Next;

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

    pub fn run(&mut self, route_start: Next) -> Result<(), RouterError>  {
        let mut point = route_start;

        loop {
            let path = point.route.route_path.clone() + "/" + &point.route.command;
            let base_path = point.route.route_path.clone() + "/*";
            let result = self.routing.routes(&path, point.clone());
            if !result.route.is_empty() {
                point = result;
            } else {
                let result = self.routing.routes(&base_path, point.clone());
                if result.route.is_empty() {
                    return Err(RouterError::Invalid("route is invalid".to_string()))
                }
                point = result;
            }
        }
    }
}
