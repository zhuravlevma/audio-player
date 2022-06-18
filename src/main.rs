extern crate core;

use app::App;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // use terminal_menu::{menu, submenu, list, button, back_button, run, mut_menu};
    // let sub = submenu("My Submenus Name", vec![
    //     string("Your Name", "Samuel", false),
    //     scroll("List", vec!["First", "Second", "Third"]),
    //     button("Exit")
    // ]);
    // let menu = menu(vec![
    //     sub,
    // ]);
    // mut_menu(&menu).get_submenu("My Submenus Name").set_selected_item_with_name("List");
    // // mut_menu(&menu).get_submenu("My Submenus Name").get_submenu("List");
    // run(&menu);
    // let response = mut_menu(&menu).get_submenu("My Submenus Name").selection_value("List").to_string();
    // println!("{}", response);
    App::new()?.launch().await?;
    Ok(())
}

mod app;
mod infra;
mod utils;
