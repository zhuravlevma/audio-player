use crate::app::player::Player;
use crate::app::time::{get_interval_secs, time_ms_now};
use crate::utils::console::ConsoleError;
use crate::views::error_view::ErrorView;
use crate::views::menu_view::MenuView;
use crate::views::playlist_view::PlaylistView;
use crate::views::track_view::TrackView;
use rodio::OutputStreamHandle;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Duration;
use terminal_menu::{mut_menu, run, TerminalMenu};
use thiserror::Error;
use crate::domains::menu_entity::{MenuEntity, MenuState, TrackState};

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

    pub fn get_sink(&self, handle: &OutputStreamHandle) -> Arc<rodio::Sink> {
        Arc::new(rodio::Sink::try_new(handle).unwrap())
    }

    pub fn append_track(&self, track: &Arc<rodio::Sink>, path: &str) {
        let file = self.get_file(path);
        track.append(rodio::Decoder::new(BufReader::new(file)).unwrap())
    }

    pub fn start(&mut self) -> Result<(), MenuError> {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let mut sink: Option<Arc<rodio::Sink>> = Option::None;
        let mut secs: u64 = 0;
        let mut current_path = String::new();
        let mut menu = MenuEntity::new();
        let start_menu = MenuView::get(&String::from(""), 0);
        let mut current = menu.run(start_menu);
        let mut start_duration: Duration = Duration::from_secs(0);
        loop {
            if !start_duration.is_zero() {
                secs = get_interval_secs(start_duration, time_ms_now());
            }
            let content = match &self.state {
                State::Main => match current.as_str() {
                    "Repeat" => {
                        self.state = State::TrackList(Track::List);
                        menu.change_state(MenuState::TrackList(TrackState::List));
                        PlaylistView::get(&current_path, secs, &self.track_list)
                    }
                    "Track list" => {
                        self.state = State::TrackList(Track::List);
                        menu.change_state(MenuState::TrackList(TrackState::List));
                        PlaylistView::get(&current_path, secs, &self.track_list)
                    }
                    "Exit" => return Ok(()),
                    _ => ErrorView::get(),
                },
                State::TrackList(track) => match track {
                    Track::List => {
                        if current.as_str() == "Back" {
                            self.state = State::Main;
                            menu.change_state(MenuState::Main);
                            MenuView::get(&current_path, secs)
                        } else if current.as_str() == "Repeat" {
                            self.state = State::TrackList(Track::List);
                            menu.change_state(MenuState::TrackList(TrackState::List));
                            PlaylistView::get(&current_path, secs, &self.track_list)
                        } else {
                            let track = self.track_list.iter().find(|&el| el == current.as_str());
                            match track {
                                None => ErrorView::get(),
                                Some(path) => match &mut sink {
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
                                },
                            }
                        }
                    }
                    Track::Play(_path) => match current.as_str() {
                        "Back" => {
                            self.state = State::TrackList(Track::List);
                            menu.change_state(MenuState::TrackList(TrackState::List));
                            PlaylistView::get(&current_path, secs, &self.track_list)
                        }
                        _ => ErrorView::get(),
                    },
                },
            };
            current = menu.run(content);
        }
    }

    pub fn get_file(&self, path: &str) -> File {
        std::fs::File::open(path).unwrap()
    }

    pub fn playing(&mut self, sink: &Arc<rodio::Sink>, path: String) -> TerminalMenu {
        self.append_track(sink, &path);
        self.player.append(sink.clone());
        self.state = State::TrackList(Track::Play(path.clone()));
        // menu.change_state(MenuState::TrackList(TrackState::Play(path.clone())));
        TrackView::get(path)
    }
}
