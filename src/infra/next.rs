use crate::app::routing::Commands;

#[derive(Clone)]
pub struct Next {
    pub command: Commands,
}

impl Next {
    pub fn new(command: Commands) -> Self {
        Self { command }
    }
}
