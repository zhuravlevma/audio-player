use terminal_menu::{button, label, menu, mut_menu, run};

pub struct TrackView {}

impl TrackView {
    pub fn getv2(path: &String) -> String {
        let t = menu(vec![
            label(format!("Track {}", path)),
            button("Pause"),
            button("Back"),
        ]);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }

    pub fn not_found() -> String {
        let t = menu(vec![label("error"), button("Back")]);
        run(&t);
        let s = mut_menu(&t).selected_item_name().to_string();
        s
    }
}
