use crate::utils::console::{Console, ConsoleError};
use thiserror::Error;

pub struct Login {
    password: String,
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Your `{0}` is incorrect")]
    Invalid(String),
    #[error("io error")]
    IoError(#[from] ConsoleError),
}

impl Login {
    pub fn new() -> Result<Self, LoginError> {
        Console::clear();
        println!("Password: ");
        let password = rpassword::read_password().unwrap();
        Ok(Self { password })
    }

    pub fn validate(&self) -> Result<(), LoginError> {
        if !self.password.eq("admin") {
            return Err(LoginError::Invalid("password".to_string()));
        }
        Console::clear();
        Ok(())
    }
}
