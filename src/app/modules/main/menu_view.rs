use terminal_menu::{button, label, menu, mut_menu, run};

pub struct MenuView {}

#[derive(Clone)]
pub enum MainMenuEvents {
    GetMenu,
    GetLocalPlaylist,
    GetExternalPlaylist,
    Exit,
}

impl MenuView {
    pub fn get_menu_with_header(track_path: &str, time: u64) -> String {
        let t = menu(vec![
            label(format!("Track {}  {} s", track_path, time)),
            label("Menu"),
            button("TrackList"),
            button("Exit"),
        ]);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }

    pub fn get_menu_without_header() -> String {
        let t = menu(vec![label("Menu"), button("TrackList"), button("Exit")]);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }
}
