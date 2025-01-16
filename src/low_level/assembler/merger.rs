use crate::low_level::assembler::Assembler;
use common_structs::message::Message;
use common_structs::types::SessionId;

impl Assembler {
    pub fn get_full_message(&mut self, session_id: SessionId) -> Option<Message> {
        let to_assemble = self.messages_to_assemble.get(&session_id)?;

        if (to_assemble.pieces.len() as u64) < to_assemble.pieces_number {
            return None;
        }

        let to_assemble = self.messages_to_assemble.remove(&session_id)?;
        Message::from_fragments(to_assemble.pieces.into_values().collect()).ok()
    }
}
