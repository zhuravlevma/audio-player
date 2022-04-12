use crate::utils::console::{Console, ConsoleError};
use std::fs;
use thiserror::Error;

pub enum State {
    Main,
    TrackList,
}

#[derive(Error, Debug)]
pub enum MenuError {
    #[error("io error")]
    IoError(#[from] ConsoleError),
}

pub struct Menu {
    state: State,
    track_list: Vec<String>,
}

impl Menu {
    pub fn new() -> Self {
        let paths = fs::read_dir("./assets").unwrap();
        let res: Vec<String> = paths
            .into_iter()
            .enumerate()
            .map(|(index, path)| {
                format!(
                    "{}  {}",
                    index + 1,
                    path.unwrap().path().display().to_string()
                )
            })
            .collect();
        Self {
            state: State::Main,
            track_list: res,
        }
    }

    fn get_main(&self) -> String {
        "Menu \n\
         1. Track list\n\
         2. Exit\n\
         "
        .to_string()
    }

    fn get_track_list(&self) -> String {
        format!(
            "Track list \n\
        {}",
            self.track_list.join(",")
        )
    }

    pub fn start(&mut self) -> Result<(), MenuError> {
        Console::clear();
        println!("{}", self.get_main());
        loop {
            let str = Console::input_line()?;
            let num: i32 = str.parse().unwrap();
            let content = match self.state {
                State::Main => match num {
                    1 => {
                        self.state = State::TrackList;
                        self.get_track_list()
                    }
                    2 => return Ok(()),
                    _ => "".to_string(),
                },
                State::TrackList => "ewe".to_string(),
            };
            Console::clear();
            println!("{}", content);
        }
    }
}
