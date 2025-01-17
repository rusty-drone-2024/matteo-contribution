mod features;
mod message_handler;

use crate::backend::network::{NetworkBacked, PacketMessage};
use crate::utils::set_panics_message;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;
use wg_2024::packet::Packet;

pub struct TextServer {
    network_backed: Option<NetworkBacked>,
    network_rcv: Receiver<PacketMessage>,
    network_send: Sender<PacketMessage>,
}

impl TextServer {
    pub fn new(packet_recv: Receiver<Packet>, packet_send: Sender<Packet>) -> Self
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
        }
    }

    pub fn run(&mut self) {
        set_panics_message("Failed server");
        let mut net_backend = self.network_backed.take().unwrap();
        thread::spawn(move || net_backend.run());

        while let Ok(packet_msg) = self.network_rcv.recv() {
            let PacketMessage(session, routing, message) = packet_msg;

            let response = self.handle_message(message.clone());
            println!(
                "----- SERVER RESPONDED [{}] TO {:?} WITH {:?}",
                session, message, response
            );

            let packet_resp = PacketMessage(session, routing.get_reversed(), response);
            let _ = self.network_send.send(packet_resp);
        }
    }
}
