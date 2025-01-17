#![allow(dead_code)]
mod ack;
mod test;
mod to_send;
mod public;

use common_structs::message::Message;
use common_structs::types::{FragmentIndex, SessionId};
use std::collections::{HashMap, HashSet};
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Fragment;

pub struct Disassembler {
    messages_to_send: HashMap<u64, DisassembledPacket>,
}

impl Disassembler {
    pub fn new() -> Self {
        Self {
            messages_to_send: HashMap::new(),
        }
    }
}

struct DisassembledPacket {
    routing: SourceRoutingHeader,
    pieces: Vec<Fragment>,
    ack_received: HashSet<FragmentIndex>,
}
