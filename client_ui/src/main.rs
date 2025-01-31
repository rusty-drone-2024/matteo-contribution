use crate::model::Model;
use crate::update::update;
use crate::view::view;
use client_bridge::send::{recv_over, send_over};
use client_bridge::{GuiRequest, GuiResponse};
use iced::Task;
use std::env;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

mod model;
mod update;
mod view;

pub fn main() {
    let addr = env::args()
        .collect::<Vec<_>>()
        .get(1)
        .expect("NO ARGUMENT")
        .to_string();

    // TODO not manual (add refresh button)
    sleep(Duration::from_millis(400));

    let mut stream = TcpStream::connect(&addr).unwrap();
    send_over(&mut stream, GuiRequest::ListAll);

    let res = recv_over::<GuiResponse>(&mut stream).unwrap();
    let GuiResponse::ListOfAll(list) = res else {
        panic!("BOIA");
    };

    let mut final_list = vec![];
    for (_, el) in list {
        final_list.extend(el.into_iter());
    }

    // TODO Handle
    let _ = iced::application("Client App", update, view)
        .run_with(|| (Model::new(addr, final_list), Task::none()));
}
