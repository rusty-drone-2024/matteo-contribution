use super::Assembler;
use common_structs::message::Message;
use common_structs::types::Session;

impl Assembler {
    /// Get the full message if it is ready. If it is it remove it from the pile.
    /// As such it must be used.
    /// # Return
    /// The optional ready message.
    #[must_use]
    pub fn take_full_message(&mut self, session: Session) -> Option<Message> {
        let to_assemble = self.messages_to_assemble.get(&session)?;

        if (to_assemble.pieces.len() as u64) < to_assemble.pieces_number {
            return None;
        }

        let to_assemble = self.messages_to_assemble.remove(&session)?;
        Message::from_fragments(to_assemble.pieces.into_values().collect()).ok()
    }
}
