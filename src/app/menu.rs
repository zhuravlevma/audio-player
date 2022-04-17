use crate::app::player::Player;
use crate::utils::console::ConsoleError;
use std::fs;
use std::io::BufReader;
use std::sync::Arc;
use terminal_menu::{button, label, menu, mut_menu, run, TerminalMenu, TerminalMenuItem};
use thiserror::Error;

pub enum Track {
    List,
    Play(String),
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
            .map(|path| path.unwrap().path().display().to_string())
            .collect();
        Self {
            state: State::Main,
            track_list: res,
            player: Player::new(),
        }
    }

    fn get_main(&self) -> TerminalMenu {
        menu(vec![label("Menu"), button("Track list"), button("Exit")])
    }

    fn get_error(&self) -> TerminalMenu {
        menu(vec![label("Error"), button("Repeat")])
    }

    fn get_track_list(&self) -> TerminalMenu {
        let mut items: Vec<TerminalMenuItem> = self.track_list.iter().map(button).collect();
        items.push(button("Back"));
        menu(items)
    }

    fn get_track(&self, path: String) -> TerminalMenu {
        menu(vec![
            label(format!("Track {}", path)),
            button("Pause"),
            button("Back"),
        ])
    }

    pub fn start(&mut self) -> Result<(), MenuError> {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let mut sink = Arc::new(rodio::Sink::try_new(&handle).unwrap());
        let start_menu = self.get_main();
        run(&start_menu);
        let mut current = mut_menu(&start_menu).selected_item_name().to_string();

        loop {
            let content = match &self.state {
                State::Main => match current.as_str() {
                    "Repeat" => {
                        self.state = State::TrackList(Track::List);
                        self.get_track_list()
                    }
                    "Track list" => {
                        self.state = State::TrackList(Track::List);
                        self.get_track_list()
                    }
                    "Exit" => return Ok(()),
                    _ => self.get_error(),
                },
                State::TrackList(track) => match track {
                    Track::List => {
                        if current.as_str() == "Back" {
                            self.state = State::Main;
                            self.get_main()
                        } else if current.as_str() == "Repeat" {
                            self.state = State::TrackList(Track::List);
                            self.get_track_list()
                        } else {
                            let track = self.track_list.iter().find(|&el| el == current.as_str());
                            match track {
                                None => self.get_error(),
                                Some(path) => {
                                    sink.stop();
                                    sink = Arc::new(rodio::Sink::try_new(&handle).unwrap());
                                    let file = std::fs::File::open(path).unwrap();
                                    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
                                    self.player.append(sink.clone());
                                    self.state = State::TrackList(Track::Play(path.clone()));
                                    self.get_track(path.clone())
                                }
                            }
                        }
                    }
                    Track::Play(_path) => match current.as_str() {
                        "Back" => {
                            self.state = State::TrackList(Track::List);
                            self.get_track_list()
                        }
                        _ => self.get_error(),
                    },
                },
            };
            run(&content);
            current = mut_menu(&content).selected_item_name().to_string();
        }
    }
}
