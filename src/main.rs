use crate::app::AppError;
use app::App;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    App::new()?.launch()?;
    Ok(())
}

mod app;
mod infra;
mod utils;
