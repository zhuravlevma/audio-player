use std::collections::HashMap;

#[derive(Clone)]
pub struct Request {
    pub body: HashMap<String, String>,
}

impl Request {
    pub fn new(body: HashMap<String, String>) -> Self {
        Self { body }
    }
}
