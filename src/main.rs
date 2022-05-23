use crate::app::AppError;
use app::App;

fn main() -> Result<(), AppError> {
    App::new()?.launch()?;
    Ok(())
}

mod app;
mod infra;
mod modules;
mod utils;
mod views;
