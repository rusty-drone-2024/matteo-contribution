#![warn(clippy::pedantic)]
mod backend;
mod client;
mod server;

use crate::server::{MediaServer, TextServer};
use client::TextMediaClient;
use common_structs::leaf::Leaf;
use crossbeam_channel::unbounded;
use std::thread;
use wg_2024::network::NodeId;

fn main() {
    let (controller_send, _) = unbounded();
    let (_, controller_rcv) = unbounded();
    let (packet_leaf_in, packet_rcv) = unbounded();
    let (packet_text_send, packet_text_leaf_out) = unbounded();
    let (packet_media_send, packet_media_leaf_out) = unbounded();

    let mut client = TextMediaClient::new(
        10,
        controller_send.clone(),
        controller_rcv.clone(),
        packet_rcv,
        vec![
            (20 as NodeId, packet_text_send),
            (30 as NodeId, packet_media_send),
        ]
        .into_iter()
        .collect(),
    );

    let mut server_text = TextServer::new(
        20,
        controller_send.clone(),
        controller_rcv.clone(),
        packet_text_leaf_out,
        vec![(10 as NodeId, packet_leaf_in.clone())]
            .into_iter()
            .collect(),
    );

    let mut server_media = MediaServer::new(
        30,
        controller_send,
        controller_rcv,
        packet_media_leaf_out,
        vec![(10 as NodeId, packet_leaf_in)].into_iter().collect(),
    );

    thread::spawn(move || server_text.run());
    thread::spawn(move || server_media.run());
    client.run();
}
