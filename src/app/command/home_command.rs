#[derive(Clone)]
pub enum HomeCommand {
    GetMenu,
    GetLocalPlaylist,
    GetNewPlaylist,
    GetPopularPlaylist,
    Exit,
}
