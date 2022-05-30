mod muzati;

use regex::Regex;
use std::error::Error;
use crate::modules::external::muzati::Muzati;

pub async fn test() -> Result<(), Box<dyn Error>> {
    let t = Muzati::new();
    let tracks = t.get_new_tracks().await?;
    for track in tracks {
        println!("{} {}", track.track_url, track.track_name)
    }
    Ok(())
}
