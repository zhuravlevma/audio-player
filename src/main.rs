use std::error::Error;
use crate::app::AppError;
use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    App::new()?.launch().await?;
    Ok(())
}

mod app;
mod infra;
mod utils;
