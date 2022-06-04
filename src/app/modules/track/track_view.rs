use terminal_menu::{button, label, menu, mut_menu, run};

pub struct TrackView {}

#[derive(Clone)]
pub enum TrackEvents {
    Pause,
    Continue,
    Back,
    PlayTrack,
}

impl TrackView {
    pub fn get_track_with_header(track_path: &str, time: u64) -> String {
        let t = menu(vec![
            label(format!("Track {}  {} s", track_path, time)),
            button("Pause"),
            button("Back"),
        ]);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }

    pub fn get_pause_track(track_path: &str, time: u64) -> String {
        let t = menu(vec![
            label(format!("Track {}  {} s", track_path, time)),
            button("Continue"),
            button("Back"),
        ]);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }

    pub fn not_found() -> String {
        let t = menu(vec![label("error"), button("Back")]);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }
}
