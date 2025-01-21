
use common_structs::message::Message;
use common_structs::types::SessionId;
use crossbeam_channel::{Receiver, Sender};
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Packet;

mod inputs;
mod packet_output;
mod thread_output;

pub use crate::backend::assembler::Assembler;
pub use crate::backend::disassembler::Disassembler;

pub struct PacketMessage(pub SessionId, pub SourceRoutingHeader, pub Message);

pub struct NetworkCommunication {
    pub backend: Option<NetworkBackend>,
    pub rcv: Receiver<PacketMessage>,
    pub send: Sender<PacketMessage>,
}

pub struct NetworkBackend {
    assembler: Assembler,
    disassembler: Disassembler,
    thread_in: Receiver<PacketMessage>,
    thread_out: Sender<PacketMessage>,
    packet_in: Receiver<Packet>,
    packet_out: Sender<Packet>,
}

impl NetworkBackend {
    pub fn new(
        thread_in: Receiver<PacketMessage>,
        thread_out: Sender<PacketMessage>,
        packet_in: Receiver<Packet>,
        packet_out: Sender<Packet>,
    ) -> Self {
        Self {
            assembler: Assembler::new(),
            disassembler: Disassembler::new(),
            thread_in,
            thread_out,
            packet_in,
            packet_out,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.read_input_and_chain();
        }
    }
}
