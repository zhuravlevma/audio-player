#[derive(Clone)]
pub struct Route {
    pub route_path: String,
    pub command: String,
}
//
// impl Route {
//     pub fn new( command: C) -> Self {
//         Self {
//             route_path: route_path.as_ref().to_string(),
//         }
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.route_path.is_empty() && self.command.is_empty()
//     }
// }
