pub enum TrackState {
    List,
    Play(String),
}

pub enum MenuState {
    Main,
    TrackList(TrackState),
}

struct MenuEntity {
    state: MenuState,
}

impl MenuEntity {
    pub fn new() -> Self {
        Self {
            state: MenuState::Main,
        }
    }
}
