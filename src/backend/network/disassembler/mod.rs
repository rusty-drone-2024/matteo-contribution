#![allow(dead_code)]
mod ack;
mod test;
mod to_send;

use common_structs::message::Message;
use common_structs::types::{FragmentIndex, SessionId};
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

    pub fn split(&mut self, session_id: SessionId, message: Message) -> Vec<Fragment> {
        self.add_message_to_send(session_id, message);
        let disassembled = self.messages_to_send.get(&session_id).unwrap();

        disassembled.pieces.clone()
    }

    pub fn ack(&mut self, session_id: SessionId, fragment_index: FragmentIndex) {
        let res = self.ack_fragment(session_id, fragment_index);

        if let Ok(true) = res {
            self.remove_message_if_acked(session_id);
        }
    }

    pub fn nack_require_resend(
        &self,
        session_id: SessionId,
        fragment_index: FragmentIndex,
    ) -> Option<Fragment> {
        let disassembled = self.messages_to_send.get(&session_id)?;
        let fragment_index = *disassembled.ack_received.get(&fragment_index)? as usize;

        disassembled.pieces.get(fragment_index).cloned()
    }
}

struct DisassembledPacket {
    pieces: Vec<Fragment>,
    ack_received: HashSet<FragmentIndex>,
}
