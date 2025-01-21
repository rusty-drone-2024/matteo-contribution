mod fragment_adder;
mod merger;
mod test;

use common_structs::message::Message;
use common_structs::types::SessionId;
use std::collections::HashMap;
use wg_2024::packet::Fragment;

pub struct Assembler {
    messages_to_assemble: HashMap<u64, MessageToAssemble>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            messages_to_assemble: HashMap::new(),
        }
    }

    pub fn merge_fragment(&mut self, session_id: SessionId, fragment: Fragment) -> Option<Message> {
        if !self.add_fragment(session_id, fragment) {
            return None;
        }

        self.get_full_message(session_id)
    }
}

struct MessageToAssemble {
    pieces_number: u64,
    pieces: HashMap<u64, Fragment>,
}

impl MessageToAssemble {
    fn new(pieces_number: u64) -> Self {
        Self {
            pieces_number,
            pieces: HashMap::default(),
        }
    }
}
