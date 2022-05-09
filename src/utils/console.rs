use thiserror::Error;

pub struct Console {}

#[derive(Error, Debug)]
pub enum ConsoleError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

impl Console {
    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }
}
