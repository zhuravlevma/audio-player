use player::player_entity::Player;

pub mod player;

pub struct Ctx {
    pub player: Player,
}

impl Ctx {
    pub fn new(player: Player) -> Self {
        Self { player }
    }
}
