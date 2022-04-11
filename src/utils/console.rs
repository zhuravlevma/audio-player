use std::io;
use thiserror::Error;


pub struct Console {}

#[derive(Error, Debug)]
pub enum ConsoleError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
}


impl Console {
    pub fn input_line() -> Result<String, ConsoleError> {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        buffer.pop();
        Ok(buffer)
    }
}