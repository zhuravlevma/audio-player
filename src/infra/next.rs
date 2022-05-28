use crate::app::routing::Commands;
use crate::infra::request::Request;

#[derive(Clone)]
pub struct Next {
    pub command: Commands,
    pub request: Option<Request>,
}

impl Next {
    pub fn new(command: Commands, request: Option<Request>) -> Self {
        Self { request, command }
    }
}
