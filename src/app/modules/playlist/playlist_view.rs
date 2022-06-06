use crate::app::command::home_command::HomeCommand;
use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::infra::request::Request;
use crate::utils::menu::Menu;
use crate::utils::view::View;
use std::collections::HashMap;
use terminal_menu::{button, label, TerminalMenuItem};

pub struct PlaylistView {}

impl PlaylistView {
    pub fn get_playlist_with_header(
        track_list: &[TrackEntity],
        track_path: &str,
        time: u64,
    ) -> Next {
        let mut items: Vec<TerminalMenuItem> =
            vec![label(format!("Track {}  {} s", track_path, time))];
        track_list
            .iter()
            .for_each(|el| items.push(button(el.get_path().to_string())));
        items.push(button("Back"));
        PlaylistView::response(Menu::create_and_handle(items))
    }

    pub fn get_playlist_without_header(track_list: &[TrackEntity]) -> Next {
        let mut items: Vec<TerminalMenuItem> = track_list
            .iter()
            .map(|el| button(el.get_path().to_string()))
            .collect();
        items.push(button("Back"));
        PlaylistView::response(Menu::create_and_handle(items))
    }
}

impl View for PlaylistView {
    fn response(command_str: String) -> Next {
        match command_str.as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetMenu), None),
            _ => Next::new(
                Commands::Playlist(PlaylistCommand::Input),
                Some(Request::new(HashMap::from([(
                    "track".to_string(),
                    command_str,
                )]))),
            ),
        }
    }
}
