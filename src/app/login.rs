use crate::utils::console::{Console, ConsoleError};
use thiserror::Error;

pub struct Login {
    username: String,
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
        println!("Please, input your username: ");
        let username = Console::input_line()?;
        println!("Password: ");
        let password = Console::input_line()?;
        Ok(Self { username, password })
    }

    pub fn validate(&self) -> Result<(), LoginError> {
        if !self.username.eq("admin") {
            return Err(LoginError::Invalid("username".to_string()));
        }
        if !self.password.eq("admin") {
            return Err(LoginError::Invalid("password".to_string()));
        }
        Console::clear();
        Ok(())
    }
}
