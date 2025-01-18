use super::Disassembler;
use common_structs::types::{FragmentIndex, SessionId};

impl Disassembler {
    pub(crate) fn ack_fragment(
        &mut self,
        session_id: SessionId,
        fragment_id: FragmentIndex,
    ) -> Result<bool, String> {
        let disassembled = self
            .messages_to_send
            .get_mut(&session_id)
            .ok_or("No session id")?;

        if fragment_id >= disassembled.pieces.len() as u64 {
            return Err("Fragment id outside bounds".to_owned());
        }

        Ok(disassembled.ack_received.insert(fragment_id))
    }

    pub(crate) fn remove_message_if_acked(&mut self, session_id: SessionId) -> bool {
        if self.is_message_acked(session_id) {
            return self.messages_to_send.remove(&session_id).is_some();
        }
        false
    }

    pub(crate) fn is_message_acked(&self, session_id: SessionId) -> bool {
        if let Some(disassembled) = self.messages_to_send.get(&session_id) {
            let success = disassembled.ack_received.len() >= disassembled.pieces.len();
            if success {
                println!(
                    "-- -- -- --DISASSEMBLER finished {} [{}/{}]",
                    session_id,
                    disassembled.ack_received.len(),
                    disassembled.pieces.len()
                );
            }
            return success;
        }
        false
    }
}
