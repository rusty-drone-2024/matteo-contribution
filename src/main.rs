#![warn(clippy::pedantic)]
mod backend;
mod client;
mod server_text;

use crate::server_text::TextServer;
use client::TextMediaClient;
use common_structs::leaf::Leaf;
use crossbeam_channel::unbounded;
use std::thread;
use wg_2024::network::NodeId;

fn main() {
    let (controller_send, _) = unbounded();
    let (_, controller_rcv) = unbounded();
    let (packet_leaf_in, packet_rcv) = unbounded();
    let (packet_send, packet_leaf_out) = unbounded();

    let mut client = TextMediaClient::new(
        10,
        controller_send.clone(),
        controller_rcv.clone(),
        packet_rcv,
        vec![(20 as NodeId, packet_send)].into_iter().collect(),
    );

    let mut server = TextServer::new(
        20,
        controller_send,
        controller_rcv,
        packet_leaf_out,
        vec![(10 as NodeId, packet_leaf_in)].into_iter().collect(),
    );

    thread::spawn(move || server.run());
    client.run();
}
