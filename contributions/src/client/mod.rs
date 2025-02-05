mod backend;
mod frontend;

use crate::client::backend::ClientBackend;
use crate::client::frontend::ClientFrontend;
use common_structs::leaf::{Leaf, LeafCommand, LeafEvent};
use crossbeam_channel::{unbounded, Receiver, Sender};
use network::NetworkBackend;
use std::collections::HashMap;
use std::thread;
use tokio_util::sync::CancellationToken;
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};

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
        controller_send: Sender<LeafEvent>,
        controller_recv: Receiver<LeafCommand>,
        packet_recv: Receiver<Packet>,
        packet_senders: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized,
    {
        let (network_in_send, network_in_rcv) = unbounded();
        let (network_out_send, network_out_rcv) = unbounded();
        let (frontend_send, frontend_rcv) = unbounded();

        let cancel_token = CancellationToken::new();

        Self {
            threads_data: Some(ThreadsData {
                network_backend: NetworkBackend::new(
                    id,
                    NodeType::Client,
                    network_in_rcv,
                    network_out_send,
                    packet_recv,
                    packet_senders,
                    controller_send,
                    controller_recv,
                ),
                backend: ClientBackend::new(
                    frontend_rcv,
                    network_out_rcv,
                    network_in_send,
                    cancel_token.clone(),
                ),
                frontend: ClientFrontend::new(id, frontend_send, cancel_token),
            }),
        }
    }

    fn run(&mut self) {
        let Some(data) = self.threads_data.take() else {
            return eprintln!("Failed to initialize");
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
