use thiserror::Error;
use login::{Login, LoginError};

pub struct App {
    login: Login,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("login error")]
    LoginError(#[from] LoginError),
}


impl App {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            login: Login::new()?,
        })
    }

    pub fn launch(&self) -> Result<(), AppError> {
        self.login.validate()?;
        Ok(())
    }
}

mod login;
mod menu;