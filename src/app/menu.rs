use crate::app::player::Player;
use crate::app::time::{get_interval_secs, time_ms_now};
use crate::domains::menu_entity::{MenuEntity, MenuState, TrackState};
use crate::domains::playlist_entity::Playlist;
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
use std::thread::sleep;
use std::time::Duration;
use terminal_menu::{mut_menu, run, TerminalMenu};
use thiserror::Error;
use crate::domains::main_menu_entity::MainMenuEntity;
use crate::domains::track_entity::TrackEntity;

#[derive(Error, Debug)]
pub enum MenuError {
    #[error("io error")]
    IoError(#[from] ConsoleError),
}

pub struct Menu {
    playlist: Playlist,
    player: Player,
    main: MainMenuEntity,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            playlist: Playlist::new("./assets"),
            main: MainMenuEntity::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), MenuError> {
        let mut player = Player::new();
        // let track = TrackEntity::new("assets/central.mp3".to_string());
        // player.play_track(&track);
        // sleep(Duration::from_secs(4));
        // player.play_track(&track);
        // sleep(Duration::from_secs(9));
        // println!("{}", player.get_current_state());
        // let r = self.playlist.get_track_list();
        let mut point = self.playlist.run(self.playlist.get_track_list());

        loop {
            let res: Vec<&str> = point.split("/|/").collect();
            point = match res[0] {
                "main" => {
                    match res[1] {
                        "Exit" => {
                            return Ok(())
                        }
                        _ => {
                            "main/|/error".to_string()
                        }
                    }
                }
                "playlist" => {
                    match res[1] {
                        "Back" => {
                            self.main.run(self.main.get_menu())
                        },
                        _ => {
                            let track = self.playlist.find_track(res[1]).unwrap();
                            self.player.play_track(track);
                            "track/|/show".to_string()
                        }
                    }
                },
                _ => "error".to_string()
            }
        }
        // println!("{}", self.playlist.run(self.playlist.get_track_list()));

        Ok(())
        //     let mut sink: Option<Arc<rodio::Sink>> = Option::None;
        //     let mut menu = MenuEntity::new();
        //     let start_menu = MenuView::get(&String::from(""), 0);
        //     let mut current = menu.run(start_menu);
        //     loop {
        //         let content = match &menu.state {
        //             MenuState::Main => match current.as_str() {
        //                 "Track list" => {
        //                     menu.change_state(MenuState::TrackList(TrackState::List));
        //                     self.playlist.get_track_list()
        //                 }
        //                 "Exit" => return Ok(()),
        //                 _ => ErrorView::get(),
        //             },
        //             MenuState::TrackList(track) => {
        //                 let mut current_path = String::new();
        //                 match track {
        //                     TrackState::List => {
        //                         if current.as_str() == "Back" {
        //                             menu.change_state(MenuState::Main);
        //                             MenuView::get(&current_path, 0)
        //                         } else {
        //                             let track = self.playlist.find_track(&current);
        //                             match track {
        //                                 None => ErrorView::get(),
        //                                 Some(track) => match &mut sink {
        //                                     None => {
        //                                         sink = Some(player.get_sink().clone());
        //                                         match sink {
        //                                             None => return Ok(()),
        //                                             Some(ref sink) => {
        //                                                 current_path = track.track_path.clone();
        //                                                 player.append_track(sink, &track.track_path);
        //                                                 player.append(sink.clone());
        //                                                 menu.change_state(MenuState::TrackList(
        //                                                     TrackState::Play(track.track_path.clone()),
        //                                                 ));
        //                                                 TrackView::get(track.track_path.clone())
        //                                             }
        //                                         }
        //                                     }
        //                                     Some(sink) => {
        //                                         sink.stop();
        //                                         *sink = player.get_sink().clone();
        //                                         current_path = track.track_path.clone();
        //                                         player.append_track(sink, &track.track_path);
        //                                         player.append(sink.clone());
        //                                         menu.change_state(MenuState::TrackList(TrackState::Play(
        //                                             track.track_path.clone(),
        //                                         )));
        //                                         TrackView::get(track.track_path.clone())
        //                                     }
        //                                 },
        //                             }
        //                         }
        //                     }
        //                     TrackState::Play(_path) => match current.as_str() {
        //                         "Back" => {
        //                             menu.change_state(MenuState::TrackList(TrackState::List));
        //                             self.playlist.get_track_list()
        //                         }
        //                         _ => ErrorView::get(),
        //                     },
        //                 }
        //             },
        //         };
        //         current = menu.run(content);
        //     }
        // }
    }
}
