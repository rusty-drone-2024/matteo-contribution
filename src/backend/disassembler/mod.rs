mod ack;
mod public;
mod test;
mod to_send;

use common_structs::types::FragmentIndex;
use std::collections::{HashMap, HashSet};
use wg_2024::network::NodeId;
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

#[derive(Clone)]
struct DisassembledPacket {
    destination: NodeId,
    pieces: Vec<Fragment>,
    ack_received: HashSet<FragmentIndex>,
}
