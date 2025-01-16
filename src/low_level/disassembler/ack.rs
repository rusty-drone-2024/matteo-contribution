use super::Disassembler;
use common_structs::types::{FragmentIndex, SessionId};

impl Disassembler {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn is_packet_acked(&self, session_id: SessionId) -> bool {
        let packet = &self.packets_to_send[&session_id];

        packet.ack_received.len() >= packet.pieces.len()
    }
}
