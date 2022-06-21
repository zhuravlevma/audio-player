use crate::utils::colors::Colors;
use crossterm::style::Color;
use terminal_menu::{back_button, button, label, menu, mut_menu, run, TerminalMenuItem};

pub struct MenuFactory {}

impl MenuFactory {
    pub fn new() -> Self {
        Self {}
    }
    pub fn create_and_handle(items: Vec<TerminalMenuItem>) -> String {
        let terminal_menu = menu(items);
        run(&terminal_menu);
        let response = mut_menu(&terminal_menu).selected_item_name().to_string();
        response
    }

    pub fn label(&self, text: &str) -> TerminalMenuItem {
        label(text)
    }

    pub fn button(&self, text: &str) -> TerminalMenuItem {
        button(text)
    }

    pub fn back_button(&self, text: &str) -> TerminalMenuItem {
        back_button(text)
    }

    pub fn set_color(&self, item: TerminalMenuItem, color: Colors) -> TerminalMenuItem {
        match color {
            Colors::Main => item.colorize(Color::Cyan),
            Colors::Label => item.colorize(Color::Magenta),
        }
    }
}
