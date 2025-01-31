use crate::model::Model;
use crate::update::update;
use crate::view::view;
use iced::Task;
use std::env;

mod model;
mod update;
mod view;


pub fn main() {
    let addr = env::args().next().expect("NO ARGUMENT");
    // TODO Handle
    let _ = iced::application("Client App", update, view)
        .run_with(|| {(Model::new(addr), Task::none())});
}
