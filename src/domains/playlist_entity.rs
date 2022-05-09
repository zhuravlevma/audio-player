use std::time::Duration;
use terminal_menu::TerminalMenu;
use crate::app::time::{get_interval_secs, time_ms_now};
use crate::domains::track_entity::TrackEntity;
use crate::views::playlist_view::PlaylistView;

struct Playlist {
    current_track: Option<TrackEntity>
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            current_track: Option::None,
        }
    }

    pub fn get_track_list(&self, track_list: &[String]) -> TerminalMenu {
        match &self.current_track {
            None => {
                PlaylistView::get("", 0, track_list)
            }
            Some(track) => {
                PlaylistView::get(&track.track_path, track.get_start(), track_list)
            }
        }
    }

    pub fn set_track(&self, track: TrackEntity) {

    }
}