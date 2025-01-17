#![allow(dead_code)]

use common_structs::message::Message;
use common_structs::types::SessionId;
use crossbeam_channel::{Receiver, Sender};
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Packet;

mod assembler;
mod disassembler;
mod inputs;
mod packet_output;
mod thread_output;

use crate::utils::set_panics_message;
pub use assembler::Assembler;
pub use disassembler::Disassembler;

pub struct PacketMessage(pub SessionId, pub SourceRoutingHeader, pub Message);

pub struct NetworkBacked {
    assembler: Assembler,
    disassembler: Disassembler,
    thread_in: Receiver<PacketMessage>,
    thread_out: Sender<PacketMessage>,
    packet_in: Receiver<Packet>,
    packet_out: Sender<Packet>,
}

impl NetworkBacked {
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
        set_panics_message("Failed network");

        loop {
            self.read_input_and_chain();
        }
    }
}
