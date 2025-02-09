#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]
//! UI process of the client application. Use iced as toolkit.
use crate::ui::{custom_theme, ClientUI, Message};
use iced::Task;

mod ui;

/// # Panics
/// if no arg is passed
pub fn loop_forever_media_gui(addr: String) {
    let title = "Matteo Text & Media Client";
    let theme = custom_theme();
    let initial_task = Task::done(Message::Refresh);
    let initialization = || (ClientUI::new(addr), initial_task);

    let _ = iced::application(title, ClientUI::update, ClientUI::view)
        .theme(move |_| theme.clone())
        .run_with(initialization);
}
