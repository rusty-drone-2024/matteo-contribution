#![allow(dead_code)]
mod ack;
mod test;
mod to_send;

use common_structs::types::FragmentIndex;
use std::collections::{HashMap, HashSet};
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
    pieces: Vec<Fragment>,
    ack_received: HashSet<FragmentIndex>,
}
