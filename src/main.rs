mod local_server;
mod low_level;
mod media_client;

use crossbeam_channel::unbounded;
use media_client::MediaClient;
use wg_2024::network::NodeId;

fn main() {
    let (controller_send, _) = unbounded();
    let (_, controller_rcv) = unbounded();
    let (_packet_leaf_in, packet_rcv) = unbounded();
    let (packet_send, _packet_leaf_out) = unbounded();

    let mut client = MediaClient::new(
        7,
        controller_send,
        controller_rcv,
        packet_rcv,
        vec![(11 as NodeId, packet_send)].into_iter().collect(),
    );

    client.run();
}
