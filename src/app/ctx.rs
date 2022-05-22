use crate::modules::player::player_entity::Player;

pub struct Ctx {
    player: Player
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }
}