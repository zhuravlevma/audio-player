use crate::app::ctx::Ctx;
use crate::app::modules::main::menu_view::MainMenuEvents;
use crate::app::modules::playlist::playlist_view::PlaylistEvents;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::modules::track::track_view::{TrackEvents, TrackView};
use crate::app::routing::Commands;
use crate::infra::next::Next;

pub struct TrackController {}

impl TrackController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_current_track(&self, _request: Next, ctx: &Ctx) -> Next {
        let response = match ctx.player.get_current_track() {
            None => TrackView::not_found(),
            Some(track) => match ctx.player.pause_time {
                None => TrackView::get_track_with_header(track.get_path(), ctx.player.get_time()),
                Some(_) => TrackView::get_pause_track(track.get_path(), ctx.player.get_time()),
            },
        };
        match response.as_ref() {
            "Back" => Next::new(Commands::Track(TrackEvents::Back), None),
            "Pause" => Next::new(Commands::Track(TrackEvents::Pause), None),
            "Continue" => Next::new(Commands::Track(TrackEvents::Continue), None),
            _ => Next::new(Commands::Track(TrackEvents::Continue), None),
        }
    }

    pub fn play_track(&self, route_data: Next, ctx: &mut Ctx) -> Next {
        match route_data.request {
            None => {}
            Some(req) => {
                let res = req.body.get("track");
                let external = req.body.get("is_external").unwrap();
                match res {
                    None => {}
                    Some(track_path) => {
                        let track_path = track_path.clone();
                        let external = external.clone().parse::<bool>().unwrap();
                        ctx.player
                            .play_track(TrackEntity::new(track_path, external))
                    }
                }
            }
        }
        Next::new(Commands::Playlist(PlaylistEvents::InputTrack), None)
    }

    pub fn pause(&self, _request: Next, ctx: &mut Ctx) -> Next {
        ctx.player.pause();
        Next::new(Commands::Playlist(PlaylistEvents::InputTrack), None)
    }

    pub fn track_continue(&self, _request: Next, ctx: &mut Ctx) -> Next {
        ctx.player.play();
        Next::new(Commands::Playlist(PlaylistEvents::InputTrack), None)
    }

    pub fn back(&self, _request: Next, _ctx: &Ctx) -> Next {
        Next::new(Commands::MainMenu(MainMenuEvents::GetLocalPlaylist), None)
    }
}
