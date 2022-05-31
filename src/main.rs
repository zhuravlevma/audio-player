use crate::app::AppError;
use crate::modules::external::test;
use app::App;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // test().await;
    App::new()?.launch()?;
    Ok(())
}

mod app;
mod infra;
mod modules;
mod utils;
mod views;
