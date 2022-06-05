use app::App;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    App::new()?.launch().await?;
    Ok(())
}

mod app;
mod infra;
mod utils;
