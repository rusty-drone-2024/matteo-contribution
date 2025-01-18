use crate::backend::network::Disassembler;
use common_structs::message::Message;
use common_structs::types::{FragmentIndex, SessionId};
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Fragment;

impl Disassembler {
    pub fn split(
        &mut self,
        session_id: SessionId,
        routing: SourceRoutingHeader,
        message: Message,
    ) -> Vec<Fragment> {
        let disassembled = self.add_message_to_send(session_id, routing, message);
        disassembled.pieces
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
    ) -> Option<(SourceRoutingHeader, Fragment)> {
        let disassembled = self.messages_to_send.get(&session_id)?;
        let fragment_index = *disassembled.ack_received.get(&fragment_index)?;
        let fragment_index = usize::try_from(fragment_index).ok()?;

        let routing = disassembled.routing.clone();
        let fragmet = disassembled.pieces.get(fragment_index).cloned()?;

        Some((routing, fragmet))
    }
}
