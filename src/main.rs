use crate::app::AppError;
use app::App;
use std::io::Cursor;

fn main() -> Result<(), AppError> {
    App::new()?.launch()?;
    let resp = reqwest::blocking::get(
        "https://s1.muzati.net/files/mp3/yarmak_-_dike_pole_(feat._alisa)_muzati.net_128.mp3",
    )
    .unwrap();
    println!("dwdwdwdwd");
    let mut cursor = Cursor::new(resp.bytes().unwrap()); // Adds Read and Seek to the bytes via Cursor
    let source = rodio::Decoder::new_flac(cursor).unwrap(); // Decoder requires it's source to impl both Read and Seek
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let r = rodio::Sink::try_new(&handle).unwrap();
    r.append(source);
    r.play();
    r.sleep_until_end();
    Ok(())
}

mod app;
mod domains;
mod utils;
mod views;
