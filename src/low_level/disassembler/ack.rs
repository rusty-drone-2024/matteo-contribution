use super::Disassembler;
use common_structs::types::{FragmentIndex, SessionId};

impl Disassembler {
    pub fn ack_fragment(
        &mut self,
        session_id: SessionId,
        fragment_id: FragmentIndex,
    ) -> Result<bool, String> {
        let packet = self
            .packets_to_send
            .get_mut(&session_id)
            .ok_or("No session id")?;

        if fragment_id >= packet.pieces.len() as u64 {
            return Err("Fragment id outside bounds".to_owned());
        }

        Ok(packet.ack_received.insert(fragment_id))
    }

    pub fn is_message_acked(&self, session_id: SessionId) -> bool {
        let packet = self.packets_to_send.get(&session_id);

        if let Some(packet) = packet {
            return packet.ack_received.len() >= packet.pieces.len();
        }
        false
    }

    pub fn remove_acked_message(&mut self, session_id: SessionId) -> bool {
        if !self.is_message_acked(session_id) {
            return false;
        }

        self.packets_to_send.remove(&session_id).is_some()
    }
}
