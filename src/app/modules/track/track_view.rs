use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::menu::Menu;
use crate::utils::view::View;
use terminal_menu::{button, label};
use crate::app::command::track_command::TrackCommand;

pub struct TrackView {}

impl TrackView {
    pub fn get_track_with_header(track_path: &str, time: u64) -> Next {
        let items = vec![
            label(format!("Track {}  {} s", track_path, time)),
            button("Pause"),
            button("Back"),
        ];
        TrackView::response(Menu::create_and_handle(items))
    }

    pub fn get_pause_track(track_path: &str, time: u64) -> Next {
        let items = vec![
            label(format!("Track {}  {} s", track_path, time)),
            button("Continue"),
            button("Back"),
        ];
        TrackView::response(Menu::create_and_handle(items))
    }

    pub fn not_found() -> Next {
        let items = vec![label("error"), button("Back")];
        TrackView::response(Menu::create_and_handle(items))
    }
}

impl View for TrackView {
    fn response(command_str: String) -> Next {
        match command_str.as_ref() {
            "Back" => Next::new(Commands::Track(TrackCommand::Back), None),
            "Pause" => Next::new(Commands::Track(TrackCommand::Pause), None),
            "Continue" => Next::new(Commands::Track(TrackCommand::Continue), None),
            _ => Next::new(Commands::NotFound, None),
        }
    }
}
