use regex::Regex;
use std::error::Error;

pub async fn test() -> Result<(), Box<dyn Error>> {
    let data = reqwest::get("https://muzati.net/music/news")
        .await?
        .text()
        .await?;
    let re = Regex::new("data-track=\"(.*?)\" data-title=\"(.*?)\"").unwrap();

    for cap in re.captures_iter(&data) {
        println!("{:?} {:?}", &cap[1], &cap[2]);
    }
    Ok(())
}
