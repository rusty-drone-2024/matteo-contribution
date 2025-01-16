mod ack;
mod to_send;

use common_structs::types::FragmentIndex;
use std::collections::{HashMap, HashSet};
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Fragment;

#[allow(dead_code)]
pub struct Disassembler {
    packets_to_send: HashMap<u64, DisassembledPacket>,
}

#[allow(dead_code)]
struct DisassembledPacket {
    routing_header: SourceRoutingHeader,
    session_id: u64,
    pieces: Vec<Fragment>,
    ack_received: HashSet<FragmentIndex>,
}
