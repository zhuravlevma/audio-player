use crate::app::ctx::Ctx;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use crate::modules::track::track_entity::TrackEntity;
use crate::views::track_view::TrackView;

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
            "Back" => Next::new(Commands::BackToPlaylist, None),
            "Pause" => Next::new(Commands::Pause, None),
            "Continue" => Next::new(Commands::Continue, None),
            _ => Next::new(Commands::BackToPlaylist, None),
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
        Next::new(Commands::ShowTrack, None)
    }

    pub fn pause(&self, _request: Next, ctx: &mut Ctx) -> Next {
        ctx.player.pause();
        Next::new(Commands::ShowTrack, None)
    }

    pub fn track_continue(&self, _request: Next, ctx: &mut Ctx) -> Next {
        ctx.player.play();
        Next::new(Commands::ShowTrack, None)
    }

    pub fn back(&self, _request: Next, _ctx: &Ctx) -> Next {
        Next::new(Commands::GetPlaylist, None)
    }
}
