use crate::app::modules::track::track_entity::TrackEntity;
use player::player_entity::Player;

pub mod player;

pub struct Ctx {
    player: Player,
}

impl Ctx {
    pub fn new(player: Player) -> Self {
        Self { player }
    }

    pub fn get_player_entity(&mut self) -> &Player {
        &self.player
    }
    pub fn get_player_entity_mut(&mut self) -> &mut Player {
        &mut self.player
    }


    pub async fn play_new_track(&mut self, track: TrackEntity) {
        self.player.play_track(track).await
    }

    pub fn pause_current_track(&mut self) {
        self.player.pause()
    }

    pub fn continue_current_track(&mut self) {
        self.player.play()
    }
}
