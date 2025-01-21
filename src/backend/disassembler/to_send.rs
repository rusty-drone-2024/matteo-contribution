use super::{DisassembledPacket, Disassembler};
use common_structs::message::Message;
use common_structs::types::SessionId;
use std::collections::HashSet;
use wg_2024::network::NodeId;

impl Disassembler {
    pub(super) fn add_message_to_send(
        &mut self,
        session_id: SessionId,
        destination: NodeId,
        message: Message,
    ) -> DisassembledPacket {
        let disassembled = DisassembledPacket {
            destination,
            pieces: message.into_fragments(),
            ack_received: HashSet::default(),
        };

        // TODO owerwrite check
        self.messages_to_send
            .insert(session_id, disassembled.clone());

        disassembled
    }
}
