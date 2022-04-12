use crate::app::AppError;
use app::App;

use std::io::BufReader;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), AppError> {
    App::new()?.launch()?;
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("assets/central.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    let data = Arc::new(sink);
    let sink_clone = data.clone();
    let t = thread::spawn(move || {
        sink_clone.sleep_until_end();
    });
    let sink = data.clone();
    // sink.set_speed(1.2);
    sink.pause();
    sink.play();
    // sink.stop();
    t.join(); /**/
    Ok(())
}

mod app;
mod utils;
