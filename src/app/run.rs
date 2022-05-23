use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::infra::router::{Router, RouterError};
use crate::utils::console::ConsoleError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunError {
    #[error("io error")]
    IoError(#[from] ConsoleError),
    #[error("route error")]
    RouteError(#[from] RouterError),
}

pub struct Run {}

impl Run {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&mut self) -> Result<(), RunError> {
        let mut router = Router::new();
        router.run(Next::new(Commands::GetPlaylist, None))?;
        Ok(())
    }
}
