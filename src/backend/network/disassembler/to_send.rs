use super::{DisassembledPacket, Disassembler};
use common_structs::message::Message;
use common_structs::types::SessionId;
use std::collections::HashSet;

impl Disassembler {
    pub(crate) fn add_message_to_send(&mut self, session_id: SessionId, message: Message) -> bool {
        let disassembled = DisassembledPacket {
            pieces: message.into_fragments(),
            ack_received: HashSet::default(),
        };

        //TODO at the moment it overwrite
        self.messages_to_send
            .insert(session_id, disassembled)
            .is_some()
    }
}
