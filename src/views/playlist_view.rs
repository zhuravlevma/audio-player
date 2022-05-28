use crate::app::ctx::Ctx;
use crate::modules::track::track_entity::TrackEntity;
use terminal_menu::{button, label, menu, mut_menu, run, TerminalMenuItem};

pub struct PlaylistView {}

impl PlaylistView {
    pub fn get_playlist_with_header(
        track_list: &[TrackEntity],
        track_path: &str,
        time: u64,
    ) -> String {
        let mut items: Vec<TerminalMenuItem> =
            vec![label(format!("Track {}  {} s", track_path, time))];
        track_list
            .iter()
            .for_each(|el| items.push(button(el.get_path().to_string())));
        items.push(button("Back"));
        let t = menu(items);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }

    pub fn get_playlist_without_header(track_list: &[TrackEntity]) -> String {
        let mut items: Vec<TerminalMenuItem> = track_list
            .iter()
            .map(|el| button(el.get_path().to_string()))
            .collect();
        items.push(button("Back"));

        let t = menu(items);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }
}
