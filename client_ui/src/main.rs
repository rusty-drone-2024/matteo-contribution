use crate::model::{ClientUI, Message};
use iced::Task;
use std::env;

mod model;
mod update;
mod view;

pub fn main() {
    let addr = env::args()
        .collect::<Vec<_>>()
        .get(1)
        .expect("NO ARGUMENT")
        .to_string();

    let title = "Matteo Text & Media Client";
    let initial_task = Task::done(Message::Refresh);
    let initialization = || (ClientUI::new(addr, vec![]), initial_task);

    let _ = iced::application(title, ClientUI::update, ClientUI::view)
        .run_with(initialization);
}