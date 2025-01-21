use crossbeam_channel::{select, Receiver, Sender};
use std::collections::HashMap;
use common_structs::leaf::{LeafCommand, LeafEvent};
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

mod inputs;
mod packet_output;
mod thread_output;

pub use crate::backend::assembler::Assembler;
pub use crate::backend::disassembler::Disassembler;
use crate::backend::topology::Topology;
use crate::backend::PacketMessage;

pub struct NetworkCommunication {
    pub backend: Option<NetworkBackend>,
    pub rcv: Receiver<PacketMessage>,
    pub send: Sender<PacketMessage>,
}

pub struct NetworkBackend {
    node_id: NodeId,
    topology: Topology,
    assembler: Assembler,
    disassembler: Disassembler,
    thread_in: Receiver<PacketMessage>,
    thread_out: Sender<PacketMessage>,
    packet_in: Receiver<Packet>,
    packets_out: HashMap<NodeId, Sender<Packet>>,
    controller_event: Sender<LeafEvent>,
    controller_command: Receiver<LeafCommand>,
}

impl NetworkBackend {
    pub fn new(
        node_id: NodeId,
        thread_in: Receiver<PacketMessage>,
        thread_out: Sender<PacketMessage>,
        packet_in: Receiver<Packet>,
        packets_out: HashMap<NodeId, Sender<Packet>>,
        controller_event: Sender<LeafEvent>,
        controller_command: Receiver<LeafCommand>,
    ) -> Self {
        Self {
            node_id,
            topology: Topology::new(node_id),
            assembler: Assembler::new(),
            disassembler: Disassembler::new(),
            thread_in,
            thread_out,
            packet_in,
            packets_out,
            controller_event,
            controller_command,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.topology.require_flood() {
                let flood_id = self.topology.take_fresh_flood_id();
                self.flood(flood_id);
            }

            let to_send = self.topology.take_sendable();
            if !to_send.is_empty() {
                for packet in to_send {
                    self.send_packet(packet);
                }
            }

            select! {
                recv(self.packet_in) -> msg => {
                    if let Ok(msg) = msg{
                        self.check_packet_and_chain(msg);
                    }
                },
                recv(self.thread_in) -> msg => {
                    if let Ok(msg) = msg{
                        self.send_message(msg);
                    }
                }
            }
        }
    }
}
