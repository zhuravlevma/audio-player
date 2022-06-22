use crate::app::command::home_command::HomeCommand;
use crate::app::command::track_command::TrackCommand;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::utils::colors::Colors;
use crate::utils::menu_factory::MenuFactory;

pub struct ExternalTrackView {
    menu_factory: MenuFactory,
}

impl ExternalTrackView {
    pub fn new(menu_factory: MenuFactory) -> Self {
        Self { menu_factory }
    }
    pub fn get_track_with_header(&self, track_name: &str, time: u64) -> Next {
        let items = vec![
            self.menu_factory.set_color(
                self.menu_factory
                    .label(&format!("♬ {} ⧗ {} s", track_name, time)),
                Colors::Label,
            ),
            self.menu_factory.button("Pause"),
            self.menu_factory.button("Download"),
            self.menu_factory.back_button("Back"),
        ];

        match MenuFactory::create_and_handle(items).as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
            "Pause" => Next::new(Commands::Track(TrackCommand::Pause)),
            "Download" => Next::new(Commands::Track(TrackCommand::Download)),
            _ => Next::new(Commands::NotFound),
        }
    }

    pub fn get_pause_track(&self, track_name: &str, time: u64) -> Next {
        let items = vec![
            self.menu_factory.set_color(
                self.menu_factory
                    .label(&format!("♬ {} ⧗ {} s", track_name, time)),
                Colors::Label,
            ),
            self.menu_factory.button("Continue"),
            self.menu_factory.button("Download"),
            self.menu_factory.back_button("Back"),
        ];
        match MenuFactory::create_and_handle(items).as_ref() {
            "Back" => Next::new(Commands::MainMenu(HomeCommand::GetNewPlaylist)),
            "Continue" => Next::new(Commands::Track(TrackCommand::Continue)),
            "Download" => Next::new(Commands::Track(TrackCommand::Download)),
            _ => Next::new(Commands::NotFound),
        }
    }
}
