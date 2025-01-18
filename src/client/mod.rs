mod client_handler;
pub mod frontend;
pub mod middleware;
mod network_msg_handler;

use crate::backend::network::{NetworkBacked, PacketMessage};
use crate::backend::{ClientNetworkRequest, ClientNetworkResponse};
use crate::utils::set_panics_message;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use std::thread;
use wg_2024::packet::Packet;

pub struct TextMediaClientBackend {
    network_backed: Option<NetworkBacked>,
    network_rcv: Receiver<PacketMessage>,
    network_send: Sender<PacketMessage>,
    client_receiver: Receiver<ClientNetworkRequest>,
    client_sender: Sender<ClientNetworkResponse>,
}

impl TextMediaClientBackend {
    pub fn new(
        packet_recv: Receiver<Packet>,
        packet_send: Sender<Packet>,
        client_receiver: Receiver<ClientNetworkRequest>,
        client_sender: Sender<ClientNetworkResponse>,
    ) -> Self
    where
        Self: Sized,
    {
        //TODO fix, at the moment we have only connection to network
        let (network_send, thread_in) = unbounded();
        let (thread_out, network_rcv) = unbounded();

        let network_backed = Some(NetworkBacked::new(
            thread_in,
            thread_out,
            packet_recv,
            packet_send,
        ));

        Self {
            network_backed,
            network_rcv,
            network_send,
            client_receiver,
            client_sender,
        }
    }

    pub fn run(&mut self) {
        set_panics_message("Failed client");
        if let Some(mut net_backend) = self.network_backed.take() {
            thread::spawn(move || net_backend.run());
        }

        loop {
            select! {
                recv(self.client_receiver) -> res => {
                    if let Ok(client_req) = res{
                        self.handle_client_request(client_req);
                    }
                },
                recv(self.network_rcv) -> res => {
                    if let Ok(packet_msg) = res{
                        self.handle_message_packet(packet_msg);
                    }
                },
            }
        }
    }
}
