use crate::app::ctx::Ctx;
use crate::app::modules::external::muzati::Muzati;
use crate::app::modules::home::home_controller::HomeController;
use crate::app::modules::playlist::playlist_controller::PlaylistController;
use crate::app::modules::playlist::playlist_repository::PlaylistRepository;
use crate::app::modules::playlist::playlist_service::Playlist;
use crate::app::modules::track::track_controller::TrackController;
use crate::infra::next::Next;
use std::error::Error;
use crate::app::command::home_command::HomeCommand;
use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::command::track_command::TrackCommand;

pub struct Routing {
    playlist_controller: PlaylistController,
    track_controller: TrackController,
    main_controller: HomeController,
}

#[derive(Clone)]
pub enum Commands {
    MainMenu(HomeCommand),
    Playlist(PlaylistCommand),
    Track(TrackCommand),
    NotFound,
}

impl Routing {
    pub fn new() -> Self {
        Self {
            playlist_controller: PlaylistController::new(Playlist::new(PlaylistRepository::new(
                "./assets",
                Muzati::new(),
            ))),
            track_controller: TrackController::new(),
            main_controller: HomeController::new(),
        }
    }

    pub async fn routes(&mut self, request: Next, ctx: &mut Ctx) -> Result<Next, Box<dyn Error>> {
        Ok(match request.command {
            Commands::MainMenu(HomeCommand::GetMenu) => {
                self.main_controller.show_menu(request, ctx)
            }
            Commands::MainMenu(HomeCommand::Exit) => self.main_controller.exit(request, ctx),
            Commands::MainMenu(HomeCommand::GetLocalPlaylist) => {
                self.playlist_controller.get_track_list(request, ctx)
            }
            Commands::Playlist(PlaylistCommand::Back) => self.playlist_controller.back(request, ctx),
            Commands::Playlist(PlaylistCommand::InputTrack) => {
                self.track_controller.get_current_track(request, ctx)
            }
            Commands::Playlist(PlaylistCommand::Input) => {
                self.playlist_controller.input(request, ctx)
            }
            Commands::MainMenu(HomeCommand::GetNewPlaylist) => {
                self.playlist_controller
                    .get_new_playlist(request, ctx)
                    .await?
            }
            Commands::Track(TrackCommand::Back) => self.track_controller.back(request, ctx),
            Commands::Track(TrackCommand::PlayTrack) => {
                self.track_controller.play_track(request, ctx)
            }
            Commands::Track(TrackCommand::Pause) => self.track_controller.pause(request, ctx),
            Commands::Track(TrackCommand::Continue) => {
                self.track_controller.track_continue(request, ctx)
            }
            Commands::NotFound => Next::new(Commands::NotFound, None),
        })
    }
}
