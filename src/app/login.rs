use cursive::{Cursive, CursiveExt};
use thiserror::Error;
use crate::utils::console::{Console, ConsoleError};
use cursive::views::{Dialog, EditView, TextView};
use cursive::traits::*;
// use crate::app::menu::State::Login;


pub struct Login {
    username: String,
    password: String,
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Your `{0}` is incorrect")]
    Invalid(String),
    #[error("io error")]
    IoError(#[from] ConsoleError),
}

impl Login {
    pub fn new() -> Result<Self, LoginError> {
        let mut siv = cursive::default();

        // Create a dialog with an edit text and a button.
        // The user can either hit the <Ok> button,
        // or press Enter on the edit text.
        siv.add_layer(
            Dialog::new()
                .title("Enter your name")
                // Padding is (left, right, top, bottom)
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    EditView::new()
                        // Call `show_popup` when the user presses `Enter`
                        .on_submit(Login::input_password)
                        // Give the `EditView` a name so we can refer to it later.
                        .with_name("name")
                        // Wrap this in a `ResizedView` with a fixed width.
                        // Do this _after_ `with_name` or the name will point to the
                        // `ResizedView` instead of `EditView`!
                        .fixed_width(20),
                )
                .button("Ok", |s| {
                    // This will run the given closure, *ONLY* if a view with the
                    // correct type and the given name is found.
                    let name = s
                        .call_on_name("name", |view: &mut EditView| {
                            // We can return content from the closure!
                            view.get_content()
                        })
                        .unwrap();

                    // Run the next step
                    Login::input_password(s, &name);
                }),
        );

        siv.run();
        println!("Please, input your username: ");
        let username = Console::input_line()?;
        println!("Password: ");
        let password = Console::input_line()?;
        Ok(Self {
            username,
            password
        })
    }

    pub fn input_password(siv: &mut Cursive, text: &str) {
        siv.run();
    }

    pub fn validate(&self) -> Result<(), LoginError> {
        if !self.username.eq("admin") {
            return Err(LoginError::Invalid("username".to_string()))
        }
        if !self.password.eq("admin") {
            return Err(LoginError::Invalid("password".to_string()))
        }
        Ok(())
    }
}