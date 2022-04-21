use crate::app::AppError;
use app::App;

struct Main {}
fn main() -> Result<(), AppError> {
    App::new()?.launch()?;
    Ok(())
}

mod app;
mod utils;
