use crate::app::command::home_command::HomeCommand;
use crate::app::ctx::player::player_entity::Player;
use crate::app::ctx::Ctx;
use crate::app::routing::{Commands, Routing};
use crate::infra::next::Next;
use crate::infra::router::{Router, RouterError};
use crate::utils::console::ConsoleError;
use std::error::Error;
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

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut router = Router::new(Routing::new());

        router
            .run(
                Next::new(Commands::MainMenu(HomeCommand::GetMenu)),
                Ctx::new(Player::new()),
            )
            .await?;
        Ok(())
    }
}
