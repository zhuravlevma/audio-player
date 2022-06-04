use std::error::Error;
use crate::app::ctx::Ctx;
use crate::app::modules::external::muzati::Muzati;
use crate::app::modules::main::main_controller::MainController;
use crate::app::modules::main::main_view::MainMenuEvents;
use crate::app::modules::playlist::playlist_controller::PlaylistController;
use crate::app::modules::playlist::playlist_service::Playlist;
use crate::app::modules::playlist::playlist_repository::PlaylistRepository;
use crate::app::modules::playlist::playlist_view::PlaylistEvents;
use crate::app::modules::track::track_controller::TrackController;
use crate::app::modules::track::track_view::TrackEvents;
use crate::infra::next::Next;

pub struct Routing {
    playlist_controller: PlaylistController,
    track_controller: TrackController,
    main_controller: MainController,
}

#[derive(Clone)]
pub enum Commands {
    MainMenu(MainMenuEvents),
    Playlist(PlaylistEvents),
    Track(TrackEvents),
}

impl Routing {
    pub fn new() -> Self {
        Self {
            playlist_controller: PlaylistController::new(Playlist::new(PlaylistRepository::new(
                "./assets",
                Muzati::new(),
            ))),
            track_controller: TrackController::new(),
            main_controller: MainController::new(),
        }
    }

    pub async fn routes(&mut self, request: Next, ctx: &mut Ctx) -> Result<Next, Box<dyn Error>> {
        Ok(match request.command {
            Commands::MainMenu(MainMenuEvents::GetMenu) => {
                self.main_controller.show_menu(request, ctx)
            }
            Commands::MainMenu(MainMenuEvents::Exit) => self.main_controller.exit(request, ctx),
            Commands::MainMenu(MainMenuEvents::GetLocalPlaylist) => {
                self.playlist_controller.get_track_list(request, ctx)
            }
            Commands::Playlist(PlaylistEvents::Back) => self.playlist_controller.back(request, ctx),
            Commands::Playlist(PlaylistEvents::InputTrack) => {
                self.track_controller.get_current_track(request, ctx)
            }
            Commands::MainMenu(MainMenuEvents::GetNewPlaylist) => self.playlist_controller.get_new_playlist(request, ctx).await?,
            Commands::Track(TrackEvents::Back) => self.track_controller.back(request, ctx),
            Commands::Track(TrackEvents::PlayTrack) => {
                self.track_controller.play_track(request, ctx)
            }
            Commands::Track(TrackEvents::Pause) => self.track_controller.pause(request, ctx),
            Commands::Track(TrackEvents::Continue) => {
                self.track_controller.track_continue(request, ctx)
            }
        })
    }
}
