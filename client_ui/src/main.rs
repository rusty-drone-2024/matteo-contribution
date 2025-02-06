#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::ui::{custom_theme, ClientUI, Message};
use iced::Task;
use std::env;

mod ui;

/// # Panics
/// if no arg is passed
pub fn main() {
    let addr = env::args().nth(1).expect("NO ARGUMENT").to_string();

    let title = "Matteo Text & Media Client";
    let theme = custom_theme();
    let initial_task = Task::done(Message::Refresh);
    let initialization = || (ClientUI::new(addr), initial_task);

    let _ = iced::application(title, ClientUI::update, ClientUI::view)
        .theme(move |_| theme.clone())
        .run_with(initialization);
}
