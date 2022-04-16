use crate::utils::console::{Console, ConsoleError};
use std::{fs, thread};
use std::io::{BufReader};
use std::sync::{Arc, Mutex};

use std::thread::{JoinHandle, sleep};
use std::time::Duration;

use thiserror::Error;
use crate::app::player::{play, Player};

pub enum Track {
    List,
    Play(usize),
}

pub enum State {
    Main,
    TrackList(Track),
}

#[derive(Error, Debug)]
pub enum MenuError {
    #[error("io error")]
    IoError(#[from] ConsoleError),
}

pub struct Menu {
    state: State,
    track_list: Vec<String>,
    player: Player,
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
            player: Player::new(),
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
            0. Back \n\
            {}",
            self.track_list.join("\n")
        )
    }

    // fn get_track(&self) -> String {
    //
    // }

    pub fn start(&mut self) -> Result<(), MenuError> {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let mut sink = Arc::new(rodio::Sink::try_new(&handle).unwrap());
        Console::clear();
        println!("{}", self.get_main());
        loop {
            let str = Console::input_line()?;
            let num: usize = str.parse().unwrap();
            println!("{}", num);
            let content = match &self.state {
                State::Main => match num {
                    1 => {
                        self.state = State::TrackList(Track::List);
                        self.get_track_list()
                    }
                    2 => return Ok(()),
                    _ => return Ok(()),
                },
                State::TrackList(track) => {
                    match track {
                        Track::List => {
                            let mut content = "".to_string();
                            if num == 0 {
                                self.state = State::Main;
                                content = self.get_main();
                            } else {
                                sink.stop();
                                sink = Arc::new(rodio::Sink::try_new(&handle).unwrap());
                                let str: Vec<&str> = self.track_list[num-1].split("  ").collect();
                                let path = str[1].to_string();
                                content = path.clone();
                                let file = std::fs::File::open(path).unwrap();
                                sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
                                self.player.append(sink.clone());
                                self.state = State::TrackList(Track::Play(num-1));
                            }
                            content
                        },
                        Track::Play(num) => {
                            let str: Vec<&str> = self.track_list[*num].split("  ").collect();
                            let path = str[1].to_string();
                            path
                        }
                    }
                }
            };
            Console::clear();
            println!("{}", content);
        }
    }
}
