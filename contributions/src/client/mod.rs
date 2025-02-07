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

/// A client that handle text and media in the form
/// of a markdown document.
pub struct TextMediaClient {
    threads_data: Option<ThreadsData>,
}

/// The three part in which this client is split.
/// Each part run on a separate thread for better performance.
struct ThreadsData {
    net_handler: NetworkBackend,
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
                net_handler: NetworkBackend::new(
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
            net_handler: net,
            backend,
            frontend,
        } = data;

        thread::spawn(move || net.loop_forever());
        thread::spawn(move || backend.loop_forever());
        frontend.loop_forever();
    }
}
