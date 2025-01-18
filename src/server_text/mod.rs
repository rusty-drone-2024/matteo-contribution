mod features;
mod message_handler;

use crate::backend::network::{NetworkBacked, NetworkCommunication, PacketMessage};
use common_structs::message::Link;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;
use wg_2024::packet::Packet;

pub struct TextServer {
    files: Vec<Link>,
    network: NetworkCommunication,
}

impl TextServer {
    pub fn new(packet_recv: Receiver<Packet>, packet_send: Sender<Packet>) -> Self
    where
        Self: Sized,
    {
        //TODO fix, at the moment we have only connection to network
        let (network_send, thread_in) = unbounded();
        let (thread_out, network_rcv) = unbounded();

        let network_backend = Some(NetworkBacked::new(
            thread_in,
            thread_out,
            packet_recv,
            packet_send,
        ));

        Self {
            files: Self::init_files(),
            network: NetworkCommunication {
                backend: network_backend,
                rcv: network_rcv,
                send: network_send,
            },
        }
    }

    pub fn run(&mut self) {
        if let Some(mut net_backend) = self.network.backend.take() {
            thread::spawn(move || net_backend.run());
        }

        while let Ok(packet_msg) = self.network.rcv.recv() {
            let PacketMessage(session, routing, message) = packet_msg;

            let response = self.handle_message(message.clone());
            println!("----- SERVER RESPONDED [{session}] TO {message:?} WITH {response:?}");

            let packet_resp = PacketMessage(session, routing.get_reversed(), response);
            let _ = self.network.send.send(packet_resp);
        }
    }
}
