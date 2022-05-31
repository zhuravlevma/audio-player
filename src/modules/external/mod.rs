use crate::modules::external::muzati::Muzati;
use std::error::Error;
use std::io::Cursor;

pub async fn test() -> Result<(), Box<dyn Error>> {
    let t = Muzati::new();
    let tracks = t.get_new_tracks().await?;
    let resp =
        reqwest::get("https://s1.muzati.net/files/mp3/koresh_-_glaza_ne_vrut_muzati.net_128.mp3")
            .await?;
    let bytes = resp.bytes().await?;
    let mut cursor = Cursor::new(bytes); // Adds Read and Seek to the bytes via Cursor
    let source = rodio::Decoder::new(cursor).unwrap();
    let handle = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle.1).unwrap();
    sink.append(source);
    sink.play();
    sink.sleep_until_end();
    for track in tracks {
        println!("{} {}", track.track_url, track.track_name)
    }
    Ok(())
}

pub mod muzati;