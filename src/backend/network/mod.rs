use common_structs::leaf::{LeafCommand, LeafEvent};
use crossbeam_channel::{select, Receiver, Sender};
use std::collections::HashMap;
use wg_2024::network::NodeId;
use wg_2024::packet::{NodeType, Packet};

mod ack_nack;
mod inputs;
mod packet_output;
mod simulation_controller;
mod thread_output;

pub use crate::backend::assembler::Assembler;
pub use crate::backend::disassembler::Disassembler;
use crate::backend::topology::Topology;
use crate::backend::PacketMessage;

pub enum NetworkOutput {
    MsgReceived(PacketMessage),
    NewLeafFound(NodeId, NodeType),
}

pub struct NetworkCommunication {
    pub backend: Option<NetworkBackend>,
    pub rcv: Receiver<NetworkOutput>,
    pub send: Sender<PacketMessage>,
}

pub struct NetworkBackend {
    node_id: NodeId,
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
            node_id,
            node_type,
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
        self.flood();

        loop {
            if self.topology.require_flood() {
                self.flood();
            }

            let to_send = self.topology.take_sendable();
            if !to_send.is_empty() {
                for packet in to_send {
                    self.send_packet(packet);
                }
            }

            select! {
                recv(self.controller_command) -> msg => {
                    let Ok(msg) = msg else {
                        continue;
                    };

                    let exit = self.handle_command(msg);
                    if exit {
                        break;
                    }
                },
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
