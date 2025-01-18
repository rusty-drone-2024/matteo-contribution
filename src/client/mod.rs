mod backend;
mod frontend;

use crate::backend::network::NetworkBackend;
use crate::client::frontend::ClientFrontend;
use crate::Leaf;
use backend::ClientBackend;
use common_structs::leaf::{LeafCommand, LeafEvent};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct TextMediaClient {
    threads_data: Option<ThreadsData>,
}

struct ThreadsData {
    network_backend: NetworkBackend,
    backend: ClientBackend,
    frontend: ClientFrontend,
}

impl Leaf for TextMediaClient {
    fn new(
        id: NodeId,
        _controller_send: Sender<LeafEvent>,
        _controller_recv: Receiver<LeafCommand>,
        packet_recv: Receiver<Packet>,
        packet_senders: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized,
    {
        let (network_in_send, network_in_rcv) = unbounded();
        let (network_out_send, network_out_rcv) = unbounded();
        let (frontend_send, frontend_rcv) = unbounded();

        // TODO remove
        let Some(packet_send) = packet_senders.into_values().next() else {
            panic!("Cannot get the single sender");
        };

        Self {
            threads_data: Some(ThreadsData {
                network_backend: NetworkBackend::new(
                    network_in_rcv,
                    network_out_send,
                    packet_recv,
                    packet_send,
                ),
                backend: ClientBackend::new(id, frontend_rcv, network_out_rcv, network_in_send),
                frontend: ClientFrontend::new(id, frontend_send),
            }),
        }
    }

    fn run(&mut self) {
        let Some(data) = self.threads_data.take() else {
            return println!("Failed to initialize");
        };

        let ThreadsData {
            mut network_backend,
            mut backend,
            frontend,
        } = data;

        thread::spawn(move || network_backend.run());
        thread::spawn(move || backend.run());
        frontend.loop_forever();
    }
}
