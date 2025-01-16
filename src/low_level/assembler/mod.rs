mod fragment_adder;
mod merger;

use std::collections::HashMap;
use common_structs::types::SessionId;
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Fragment;

#[allow(dead_code)]
pub struct Assembler {
    messages_to_assemble: HashMap<u64, MessageToAssemble>,
}

#[allow(dead_code)]
struct MessageToAssemble {
    session_id: u64,
    pieces_number: u64,
    pieces: HashMap<u64, Fragment>,
}

impl MessageToAssemble{
    fn new(session_id: SessionId, pieces_number: u64) -> Self{
        Self{
            session_id,
            pieces_number,
            pieces: HashMap::default(),
        }
    }
}