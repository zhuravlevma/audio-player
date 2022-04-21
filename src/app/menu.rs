use crate::app::player::Player;
use crate::utils::console::ConsoleError;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Sink};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rodio::OutputStreamHandle;
use terminal_menu::{button, label, menu, mut_menu, run, TerminalMenu, TerminalMenuItem};
use thiserror::Error;
use crate::app::time::{get_interval_secs, time_ms_now};
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

    fn get_main(&self, current_track: &str, s: u64) -> TerminalMenu {
        if s != 0 {
            return menu(vec![
                label(format!("Track {}  {} s", current_track, s)),
                label("Menu"),
                button("Track list"),
                button("Exit"),
            ]);
        }
        menu(vec![label("Menu"), button("Track list"), button("Exit")])
    }

    fn get_error(&self) -> TerminalMenu {
        menu(vec![label("Error"), button("Repeat")])
    }

    fn get_track_list(&self, current_track: &str, s: u64) -> TerminalMenu {
        if s != 0 {
            let mut items: Vec<TerminalMenuItem> =
                vec![label(format!("Track {}  {} s", current_track, s))];
            self.track_list.iter().for_each(|el| items.push(button(el)));
            items.push(button("Back"));
            return menu(items);
        }
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

    pub fn get_sink(&self, handle: &OutputStreamHandle) -> Arc<rodio::Sink> {
        Arc::new(rodio::Sink::try_new(&handle).unwrap())
    }

    pub fn start_menu(&self) -> TerminalMenu {
        let start_menu = self.get_main(&String::new(), 0);
        run(&start_menu);
        start_menu
    }

    pub fn append_track(&self, track: &Arc<rodio::Sink>, path: &str) {
        let file = self.get_file(path);
        track.append(rodio::Decoder::new(BufReader::new(file)).unwrap())
    }

    pub fn start(&mut self) -> Result<(), MenuError> {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let mut sink:Option<Arc<rodio::Sink>> = Option::None;
        let mut secs: u64 = 0;
        let mut current_path = String::new();
        let start_menu = self.start_menu();
        let mut current = mut_menu(&start_menu).selected_item_name().to_string();
        let mut start_duration: Duration = Duration::from_secs(0);
        loop {
            if !start_duration.is_zero() {
                secs = get_interval_secs(start_duration, time_ms_now());
            }
            let content = match &self.state {
                State::Main => match current.as_str() {
                    "Repeat" => {
                        self.state = State::TrackList(Track::List);
                        self.get_track_list(&current_path, secs)
                    }
                    "Track list" => {
                        self.state = State::TrackList(Track::List);
                        self.get_track_list(&current_path, secs)
                    }
                    "Exit" => return Ok(()),
                    _ => self.get_error(),
                },
                State::TrackList(track) => match track {
                    Track::List => {
                        if current.as_str() == "Back" {
                            self.state = State::Main;
                            self.get_main(&current_path, secs)
                        } else if current.as_str() == "Repeat" {
                            self.state = State::TrackList(Track::List);
                            self.get_track_list(&current_path, secs)
                        } else {
                            let track = self.track_list.iter().find(|&el| el == current.as_str());
                            match track {
                                None => self.get_error(),
                                Some(path) => {
                                    match &mut sink {
                                        None => {
                                            sink = Some(self.get_sink(&handle));
                                            match sink {
                                                None => return Ok(()),
                                                Some(ref sink) => {
                                                    start_duration = time_ms_now();
                                                    current_path = path.clone();
                                                    self.playing(sink, path.clone())
                                                }
                                            }
                                        }
                                        Some(sink) => {
                                            sink.stop();
                                            *sink = self.get_sink(&handle);
                                            start_duration = time_ms_now();
                                            current_path = path.clone();
                                            self.playing(sink, path.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Track::Play(_path) => match current.as_str() {
                        "Back" => {
                            self.state = State::TrackList(Track::List);
                            self.get_track_list(&current_path, secs)
                        }
                        _ => self.get_error(),
                    },
                },
            };
            run(&content);
            current = mut_menu(&content).selected_item_name().to_string();
        }
    }

    pub fn get_file(&self, path: &str) -> File {
        std::fs::File::open(path).unwrap()
    }

    pub fn playing(&mut self, sink: &Arc<rodio::Sink>, path: String) -> TerminalMenu {
        self.append_track(sink, &path);
        self.player.append(sink.clone());
        self.state = State::TrackList(Track::Play(path.clone()));
        self.get_track(path.clone())
    }
}
