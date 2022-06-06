use crate::app::run::{Run, RunError};
use modules::auth::login::{Login, LoginError};
use std::error::Error;
use thiserror::Error;

pub struct App {
    login: Login,
    routing: Run,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("login error")]
    LoginError(#[from] LoginError),
    #[error("login error")]
    MenuError(#[from] RunError),
}

impl App {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            login: Login::new()?,
            routing: Run::new(),
        })
    }

    pub async fn launch(&mut self) -> Result<(), Box<dyn Error>> {
        self.login.validate()?;
        self.routing.start().await?;
        Ok(())
    }
}

pub mod command;
pub mod ctx;
pub mod modules;
pub mod routing;
mod run;
