mod command;
mod errors;
mod packet_in;
mod packet_out;
mod test;
mod thread_out;
mod utils;

use crate::assembler::Assembler;
use crate::disassembler::Disassembler;
use crate::topology::Topology;
use crate::PacketMessage;
use common_structs::leaf::{LeafCommand, LeafEvent};
use crossbeam_channel::{select, Receiver, Sender};
use std::collections::HashMap;
use std::time::Duration;
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};

pub enum NetworkOutput {
    MsgReceived(PacketMessage),
    NewLeafFound(NodeId, NodeType),
}

pub struct NetworkCommunication {
    pub backend: Option<NetworkBackend>,
    pub receiver: Receiver<NetworkOutput>,
    pub sender: Sender<PacketMessage>,
}

pub struct NetworkBackend {
    id: NodeId,
    node_type: NodeType,
    topology: Topology,
    assembler: Assembler,
    disassembler: Disassembler,
    thread_in: Receiver<PacketMessage>,
    thread_out: Sender<NetworkOutput>,
    packet_in: Receiver<Packet>,
    packets_out: HashMap<NodeId, Sender<Packet>>,
    controller_event: Sender<LeafEvent>,
    controller_command: Receiver<LeafCommand>,
}

impl NetworkBackend {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        node_id: NodeId,
        node_type: NodeType,
        thread_in: Receiver<PacketMessage>,
        thread_out: Sender<NetworkOutput>,
        packet_in: Receiver<Packet>,
        packets_out: HashMap<NodeId, Sender<Packet>>,
        controller_event: Sender<LeafEvent>,
        controller_command: Receiver<LeafCommand>,
    ) -> Self {
        Self {
            id: node_id,
            node_type,
            topology: Topology::new(node_id),
            assembler: Assembler::default(),
            disassembler: Disassembler::default(),
            thread_in,
            thread_out,
            packet_in,
            packets_out,
            controller_event,
            controller_command,
        }
    }

    pub fn loop_forever(mut self) {
        let mut exit = false;
        self.flood();

        while !exit {
            select! {
                recv(self.controller_command) -> msg => {
                    let Ok(comm) = msg else { continue; };
                    exit = Self::handle_command(&mut self.packets_out, &mut self.topology, comm);
                },
                recv(self.packet_in) -> msg => {
                    let Ok(packet) = msg else { continue; };

                    if let Some(error) = Self::find_routing_error(self.id, &packet) {
                        self.handle_error(packet, error);
                        continue;
                    }

                    self.handle_packet(packet);
                    self.send_if_possible();
                },
                recv(self.thread_in) -> msg => {
                    let Ok(msg) = msg else { continue; };
                    self.send_message(msg);
                    self.flood_if_needed(false);
                }
                default(Duration::from_secs(1)) => {
                    self.flood_if_needed(true);
                }
            }
        }
    }

    fn flood_if_needed(&mut self, aggressive: bool) {
        if self.disassembler.require_flood(aggressive) {
            self.flood();
        }
    }

    fn send_if_possible(&mut self) {
        let to_send = self.disassembler.take_ready_session();

        for session in to_send {
            self.send_split(session);
        }
    }
}
