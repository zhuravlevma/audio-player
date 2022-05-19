use crate::infra::request::Request;
use crate::infra::route::Route;

#[derive(Clone)]
pub struct Next {
    pub route: Route,
    pub request: Option<Request>,
}

impl Next {
    pub fn new(route: Route, request: Option<Request>) -> Self {
        Self { request, route }
    }
}
