use super::Assembler;
use common_structs::message::Message;
use common_structs::types::Session;
use wg_2024::packet::Fragment;

impl Assembler {
    pub fn merge_fragment(&mut self, session: Session, fragment: Fragment) -> Option<Message> {
        if !self.add_fragment(session, fragment) {
            return None;
        }

        self.get_full_message(session)
    }

    pub(super) fn get_full_message(&mut self, session: Session) -> Option<Message> {
        let to_assemble = self.messages_to_assemble.get(&session)?;

        if (to_assemble.pieces.len() as u64) < to_assemble.pieces_number {
            return None;
        }

        let to_assemble = self.messages_to_assemble.remove(&session)?;
        Message::from_fragments(to_assemble.pieces.into_values().collect()).ok()
    }
}
