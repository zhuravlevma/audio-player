use terminal_menu::{button, label, menu, mut_menu, run};

pub struct MenuView {}

impl MenuView {
    pub fn get(current_track: &str, s: u64) -> String {
        if s != 0 {
            let t = menu(vec![
                label(format!("Track {}  {} s", current_track, s)),
                label("Menu"),
                button("Track list"),
                button("Exit"),
            ]);
            run(&t);
            let s = mut_menu(&t).selected_item_name().to_string();
            return s;
        }
        let t = menu(vec![label("Menu"), button("Track list"), button("Exit")]);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }
}
