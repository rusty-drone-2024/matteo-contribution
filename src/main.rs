mod local_server;
mod low_level;
mod media_client;

use common_structs::leaf::Leaf;
use crossbeam_channel::unbounded;
use low_level::test_responder::EchoServer;
use media_client::MediaClient;
use std::thread;
use wg_2024::network::NodeId;

fn main() {
    let (controller_send, _) = unbounded();
    let (_, controller_rcv) = unbounded();
    let (packet_leaf_in, packet_rcv) = unbounded();
    let (packet_send, packet_leaf_out) = unbounded();

    let mut client = MediaClient::new(
        7,
        controller_send,
        controller_rcv,
        packet_rcv,
        vec![(20 as NodeId, packet_send)].into_iter().collect(),
    );

    let mut echo_server = EchoServer::new(packet_leaf_out, packet_leaf_in);

    thread::spawn(move || {
        echo_server.run();
    });

    client.run();
}
