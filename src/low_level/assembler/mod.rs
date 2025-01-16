use std::collections::HashMap;
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Fragment;

#[allow(dead_code)]
pub struct Assembler {
    expecting_session_ids: HashMap<u64, DisassembledPacked>,
}

#[allow(dead_code)]
struct DisassembledPacked {
    routing_header: SourceRoutingHeader,
    session_id: u64,
    pieces: HashMap<u64, Fragment>,
}
