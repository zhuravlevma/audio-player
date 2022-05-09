use terminal_menu::{mut_menu, run, TerminalMenu};
use crate::domains::track_entity::TrackEntity;
use crate::views::menu_view::MenuView;

pub enum TrackState {
    List,
    Play(String),
}

pub enum MenuState {
    Main,
    TrackList(TrackState),
}

pub struct MenuEntity {
    pub state: MenuState,
}

impl MenuEntity {
    pub fn new() -> Self {
        Self {
            state: MenuState::Main,
        }
    }

    pub fn run(&self, terminal_menu: TerminalMenu) -> String {
        run(&terminal_menu);
        mut_menu(&terminal_menu).selected_item_name().to_string()
    }

    pub fn change_state(&mut self, new_state: MenuState) {
        self.state = new_state;
    }
}
