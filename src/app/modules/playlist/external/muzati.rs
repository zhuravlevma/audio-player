use regex::Regex;
use std::error::Error;

pub struct Muzati {
    base_url: String,
    new_tracks: Option<Vec<MuzatiDto>>,
}

#[derive(Clone)]
pub struct MuzatiDto {
    pub track_url: String,
    pub track_name: String,
}

impl Muzati {
    pub fn new() -> Self {
        Self {
            base_url: String::from("https://muzati.net"),
            new_tracks: None,
        }
    }

    pub async fn get_new_tracks(&mut self) -> Result<Vec<MuzatiDto>, Box<dyn Error>> {
        let tracks = match &self.new_tracks {
            None => {
                let re = Regex::new("data-track=\"(.*?)\" data-title=\"(.*?)\"").unwrap();
                let html_page = reqwest::get(format!("{}/music/news", self.base_url))
                    .await?
                    .text()
                    .await?;

                let tracks: Vec<MuzatiDto> = re
                    .captures_iter(&html_page)
                    .map(|el| MuzatiDto {
                        track_url: el[1].to_string(),
                        track_name: el[2].to_string(),
                    })
                    .collect();
                self.new_tracks = Some(tracks.clone());
                tracks
            }
            Some(tracks) => tracks.clone(),
        };
        Ok(tracks)
    }
}
