use crate::model::{Message, Model};
use crate::update::update;
use crate::view::view;
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

    // TODO Handle
    let _ = iced::application("Client App", update, view)
        .run_with(|| (Model::new(addr, vec!()), Task::done(Message::Refresh)));
}
