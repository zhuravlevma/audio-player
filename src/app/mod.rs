use crate::app::menu::{Menu, MenuError};
use login::{Login, LoginError};
use thiserror::Error;

pub struct App {
    login: Login,
    menu: Menu,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("login error")]
    LoginError(#[from] LoginError),
    #[error("login error")]
    MenuError(#[from] MenuError),
}

impl App {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            login: Login::new()?,
            menu: Menu::new(),
        })
    }

    pub fn launch(&mut self) -> Result<(), AppError> {
        self.login.validate()?;
        self.menu.start()?;
        Ok(())
    }
}

mod login;
mod menu;
mod player;
pub(crate) mod time;
