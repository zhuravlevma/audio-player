use crate::modules::player::player_entity::Player;

pub struct Ctx {
    pub player: Player,
}

impl Ctx {
    pub fn new(player: Player) -> Self {
        Self {
            player,
        }
    }
}
