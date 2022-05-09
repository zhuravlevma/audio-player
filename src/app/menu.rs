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
use crate::domains::playlist_entity::Playlist;

#[derive(Error, Debug)]
pub enum MenuError {
    #[error("io error")]
    IoError(#[from] ConsoleError),
}

pub struct Menu {
    playlist: Playlist,
    player: Player,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            playlist: Playlist::new("./assets"),
        }
    }

    pub fn get_sink(&self, handle: &OutputStreamHandle) -> Arc<rodio::Sink> {
        Arc::new(rodio::Sink::try_new(handle).unwrap())
    }

    pub fn append_track(&self, track: &Arc<rodio::Sink>, path: &str) {
        let file = std::fs::File::open(path).unwrap();
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
            let content = match &menu.state {
                MenuState::Main => match current.as_str() {
                    "Track list" => {
                        menu.change_state(MenuState::TrackList(TrackState::List));
                        self.playlist.get_track_list()
                    }
                    "Exit" => return Ok(()),
                    _ => ErrorView::get(),
                },
                MenuState::TrackList(track) => match track {
                    TrackState::List => {
                        if current.as_str() == "Back" {
                            menu.change_state(MenuState::Main);
                            MenuView::get(&current_path, secs)
                        } else {
                            let track = self.playlist.find_track(&current);
                            match track {
                                None => ErrorView::get(),
                                Some(track) => match &mut sink {
                                    None => {
                                        sink = Some(self.get_sink(&handle));
                                        match sink {
                                            None => return Ok(()),
                                            Some(ref sink) => {
                                                start_duration = time_ms_now();
                                                current_path = track.track_path.clone();
                                                let path = track.track_path.clone();
                                                self.append_track(sink, &path);
                                                self.player.append(sink.clone());
                                                menu.change_state(MenuState::TrackList(TrackState::Play(path.clone())));
                                                TrackView::get(path)
                                            }
                                        }
                                    }
                                    Some(sink) => {
                                        sink.stop();
                                        *sink = self.get_sink(&handle);
                                        start_duration = time_ms_now();
                                        current_path = track.track_path.clone();
                                        self.append_track(sink, &current_path);
                                        self.player.append(sink.clone());
                                        menu.change_state(MenuState::TrackList(TrackState::Play(current_path.clone())));
                                        TrackView::get(current_path.clone())
                                    }
                                },
                            }
                        }
                    }
                    TrackState::Play(_path) => match current.as_str() {
                        "Back" => {
                            menu.change_state(MenuState::TrackList(TrackState::List));
                            self.playlist.get_track_list()
                        }
                        _ => ErrorView::get(),
                    },
                },
            };
            current = menu.run(content);
        }
    }
}
