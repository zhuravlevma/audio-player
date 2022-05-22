use crate::infra::next::Next;
use crate::infra::route::Route;
use crate::modules::main::main_controller::MainController;
use crate::modules::player::player_entity::Player;
use crate::modules::playlist::playlist_controller::PlaylistController;
use crate::modules::playlist::playlist_entity::Playlist;
use crate::modules::track::track_controller::TrackController;

pub struct Routing {
    playlist_controller: PlaylistController,
    track_controller: TrackController,
    main_controller: MainController,
    player: Player,
}

#[derive(Clone)]
pub enum Commands {
    GetMainMenu,
    GetTrackList,
    Exit,
    GetPlaylist,
    BackToMain,
    ShowTrack,
    BackToPlaylist,
    PlayTrack
}

impl Routing {
    pub fn new() -> Self {
        Self {
            playlist_controller: PlaylistController::new(Playlist::new("./assets")),
            track_controller: TrackController::new(),
            main_controller: MainController::new(),
            player: Player::new(),
        }
    }

    pub fn routes(&mut self, path: Commands, route: Next) -> Next {
        match path {
            Commands::GetMainMenu => self.main_controller.show_menu(),
            Commands::GetTrackList => self.main_controller.playlist(),
            Commands::Exit => self.main_controller.exit(),
            Commands::GetPlaylist => self.playlist_controller.get_track_list(route),
            Commands::BackToMain => self.playlist_controller.back(),
            Commands::ShowTrack => self.track_controller.get_current_track(&self.player),
            Commands::BackToPlaylist => self.track_controller.back(),
            Commands::PlayTrack => self.track_controller.play_track(route, &mut self.player)
        }
    }
}
