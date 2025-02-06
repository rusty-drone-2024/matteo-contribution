#![cfg(test)]

use crate::backend::{NetworkBackend, NetworkOutput};
use crate::PacketMessage;
use common_structs::leaf::{LeafCommand, LeafEvent};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::time::Duration;
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};

type Net = NetworkBackend;
const TIMEOUT: Duration = Duration::from_millis(50);

mod command;
mod errors;
mod integration;
mod utils;

#[allow(unused)]
struct Connections {
    net_in: Sender<PacketMessage>,
    net_out: Receiver<NetworkOutput>,
    pck_in: Sender<Packet>,
    pck_out: Receiver<Packet>,
    leaf_event_out: Receiver<LeafEvent>,
    leaf_command_in: Sender<LeafCommand>,
}

impl Connections {
    fn new(
        net_in: Sender<PacketMessage>,
        net_out: Receiver<NetworkOutput>,
        pck_in: Sender<Packet>,
        pck_out: Receiver<Packet>,
        leaf_event_out: Receiver<LeafEvent>,
        leaf_command_in: Sender<LeafCommand>,
    ) -> Self {
        Self {
            net_in,
            net_out,
            pck_in,
            pck_out,
            leaf_event_out,
            leaf_command_in,
        }
    }
}

fn new_net_handler(id: NodeId, next_id: NodeId) -> (Net, Connections) {
    let net_in = unbounded();
    let net_out = unbounded();
    let pck_in = unbounded();
    let pck_out = unbounded();
    let leaf_event = unbounded();
    let leaf_command = unbounded();

    let net = Net::new(
        id,
        NodeType::Client,
        net_in.1,
        net_out.0,
        pck_in.1,
        [(next_id, pck_out.0)].into_iter().collect(),
        leaf_event.0,
        leaf_command.1,
    );

    let conn = Connections::new(
        net_in.0,
        net_out.1,
        pck_in.0,
        pck_out.1,
        leaf_event.1,
        leaf_command.0,
    );

    (net, conn)
}
