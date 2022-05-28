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
        match ctx.player.get_current_trackv2() {
            None => {
                let s = TrackView::not_found();
                match s.as_ref() {
                    "Back" => Next::new(Commands::BackToPlaylist, None),
                    _ => Next::new(Commands::BackToPlaylist, None),
                }
            }
            Some(track) => {
                let s = TrackView::getv2(track.get_path());
                match s.as_ref() {
                    "Back" => Next::new(Commands::BackToPlaylist, None),
                    _ => Next::new(Commands::BackToPlaylist, None),
                }
            }
        }
    }

    pub fn play_track(&self, route_data: Next, ctx: &mut Ctx) -> Next {
        match route_data.request {
            None => {}
            Some(req) => {
                let res = req.body.get("track");
                match res {
                    None => {}
                    Some(track_path) => {
                        let track_path = track_path.clone();
                        ctx.player.play_track(&TrackEntity::new(track_path))
                    }
                }
            }
        }
        Next::new(Commands::ShowTrack, None)
    }

    pub fn back(&self, _request: Next, _ctx: &Ctx) -> Next {
        Next::new(Commands::GetPlaylist, None)
    }
}
