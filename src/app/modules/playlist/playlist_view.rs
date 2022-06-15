use crate::app::command::home_command::HomeCommand;
use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::ctx::player::player_entity::Player;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::menu::Menu;
use terminal_menu::{button, label, TerminalMenuItem};

pub struct PlaylistView {}

impl PlaylistView {
    pub fn get_playlist(player: &Player, tracks: &[TrackEntity]) -> Next {
        match player.get_current_track() {
            None => PlaylistView::get_playlist_without_header(tracks),
            Some(track) => {
                PlaylistView::get_playlist_with_header(tracks, track.get_name(), player.get_time())
            }
        }
    }
    pub fn get_playlist_with_header(
        track_list: &[TrackEntity],
        track_name: &str,
        time: u64,
    ) -> Next {
        let mut items: Vec<TerminalMenuItem> =
            vec![label(format!("Track {}  {} s", track_name, time))];
        track_list
            .iter()
            .for_each(|el| items.push(button(el.get_name())));
        items.push(button("Back"));
        let track_name = Menu::create_and_handle(items);
        match track_name.as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetMenu)),
            _ => {
                let track = track_list
                    .iter()
                    .find(|el| el.get_name().eq(&track_name))
                    .unwrap();
                Next::new(Commands::Playlist(PlaylistCommand::Input(track.clone())))
            }
        }
    }

    pub fn get_playlist_without_header(track_list: &[TrackEntity]) -> Next {
        let mut items: Vec<TerminalMenuItem> =
            track_list.iter().map(|el| button(el.get_name())).collect();
        items.push(button("Back"));
        let track_name = Menu::create_and_handle(items);
        match track_name.as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetMenu)),
            _ => {
                let track = track_list
                    .iter()
                    .find(|el| el.get_name().eq(&track_name))
                    .unwrap();
                Next::new(Commands::Playlist(PlaylistCommand::Input(track.clone())))
            }
        }
    }
}
