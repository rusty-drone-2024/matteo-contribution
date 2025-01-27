mod features;
mod message_handler;

use crate::backend::network::NetworkOutput::MsgReceived;
use crate::backend::network::{NetworkBackend, NetworkCommunication};
use crate::backend::PacketMessage;
use common_structs::leaf::{Leaf, LeafCommand, LeafEvent};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};

pub struct MediaServer {
    #[allow(dead_code)]
    node_id: NodeId,
    network: NetworkCommunication,
}

impl Leaf for MediaServer {
    fn new(
        id: NodeId,
        controller_send: Sender<LeafEvent>,
        controller_recv: Receiver<LeafCommand>,
        packet_recv: Receiver<Packet>,
        packets_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized,
    {
        let (network_send, thread_in) = unbounded();
        let (thread_out, network_rcv) = unbounded();

        let network_backend = Some(NetworkBackend::new(
            id,
            NodeType::Server,
            thread_in,
            thread_out,
            packet_recv,
            packets_send,
            controller_send,
            controller_recv,
        ));

        Self {
            node_id: id,
            network: NetworkCommunication {
                backend: network_backend,
                rcv: network_rcv,
                send: network_send,
            },
        }
    }

    fn run(&mut self) {
        if let Some(mut net_backend) = self.network.backend.take() {
            thread::spawn(move || net_backend.run());
        }

        while let Ok(net_msg) = self.network.rcv.recv() {
            let MsgReceived(packet_msg) = net_msg else {
                continue; // Ignore update of network
            };
            let PacketMessage {
                session,
                opposite_end,
                message,
            } = packet_msg;

            let response = Self::handle_test_message(message.clone());

            let packet_resp = PacketMessage::new(session, opposite_end, response);
            let _ = self.network.send.send(packet_resp);
        }
    }
}
