pub struct Route {
    pub route_path: String,
    pub command: String,
}

// impl<T> Route<T> {
//     pub fn new<T>(route_path: String, command: String, data: T) -> Self<T> {
//         Self {
//             route_path,
//             command,
//             data
//         }
//     }
// }

impl Route{
    pub fn new<R: AsRef<str>, C: AsRef<str>>(route_path: R, command: C) -> Self {
        Self {
            route_path: route_path.as_ref().to_string(),
            command: command.as_ref().to_string(),
        }
    }
}