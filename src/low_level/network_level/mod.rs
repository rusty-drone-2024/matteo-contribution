#![allow(dead_code)]
mod channels;
mod flood;
mod handle_client;
mod handle_packet;
mod topology;

use crate::low_level::fragment_level::{Assembler, Disassembler};
use crate::low_level::{ClientNetworkRequest, ClientNetworkResponse};
use crossbeam_channel::{select, Receiver, Sender};
use std::collections::HashMap;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct NetworkHandler {
    assembler: Assembler,
    disassembler: Disassembler,
    client_receiver: Receiver<ClientNetworkRequest>,
    client_sender: Sender<ClientNetworkResponse>,
    packet_receiver: Receiver<Packet>,
    packet_senders: HashMap<NodeId, Sender<Packet>>,
}

impl NetworkHandler {
    pub fn new(
        client_receiver: Receiver<ClientNetworkRequest>,
        client_sender: Sender<ClientNetworkResponse>,
        packet_receiver: Receiver<Packet>,
        packet_senders: HashMap<NodeId, Sender<Packet>>,
    ) -> Self {
        Self {
            assembler: Assembler::new(),
            disassembler: Disassembler::new(),
            client_receiver,
            client_sender,
            packet_receiver,
            packet_senders,
        }
    }

    pub fn run(&mut self) {
        loop {
            select! {
                recv(self.client_receiver) -> res => {
                    if let Ok(client_req) = res{
                        self.handle_client_request(client_req);
                    }
                },
                recv(self.packet_receiver) -> res => {
                    if let Ok(packet) = res{
                        self.handle_packet(packet)
                    }
                },
            }
        }
    }
}
