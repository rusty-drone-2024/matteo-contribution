mod message_handler;

use common_structs::leaf::{Leaf, LeafCommand, LeafEvent};
use crossbeam_channel::{unbounded, Receiver, Sender};
use network::NetworkOutput::MsgReceived;
use network::{NetworkBackend, NetworkCommunication};
use std::collections::HashMap;
use std::thread;
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};

/// A media server with some test data.
pub struct MediaServer {
    net: NetworkCommunication,
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
            net: NetworkCommunication {
                backend: network_backend,
                receiver: network_rcv,
                sender: network_send,
            },
        }
    }

    fn run(&mut self) {
        if let Some(net_backend) = self.net.backend.take() {
            thread::spawn(move || net_backend.loop_forever());
        }

        while let Ok(net_msg) = self.net.receiver.recv() {
            let MsgReceived(packet_msg) = net_msg else {
                continue; // Ignore update of backend
            };

            let response = Self::handle_message(packet_msg);
            let _ = self.net.sender.send(response);
        }
    }
}
