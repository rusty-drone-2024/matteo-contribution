#![warn(clippy::pedantic)]
mod backend;
mod client;
mod server_text;

use crate::server_text::TextServer;
use client::TextMediaClient;
use common_structs::leaf::Leaf;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

fn main() {
    let (controller_send, _) = unbounded();
    let (_, controller_rcv) = unbounded();
    let (packet_leaf_in, packet_rcv) = unbounded();
    let (packet_send, packet_leaf_out) = unbounded();

    let mut client = TextMediaClient::new(
        7,
        controller_send,
        controller_rcv,
        packet_rcv,
        vec![(20 as NodeId, packet_send)].into_iter().collect(),
    );

    run_test_server(packet_leaf_out, packet_leaf_in);
    client.run();
}

fn run_test_server(receiver: Receiver<Packet>, sender: Sender<Packet>) {
    let mut echo_server = TextServer::new(receiver, sender);

    thread::spawn(move || {
        echo_server.run();
    });
}
