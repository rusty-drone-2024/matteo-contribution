use crate::backend::network::{NetworkBacked, PacketMessage};
use common_structs::types::SessionId;
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Fragment;

impl NetworkBacked {
    pub(super) fn merger_and_chain(
        &mut self,
        session_id: SessionId,
        routing_header: SourceRoutingHeader,
        fragment: Fragment,
    ) {
        if let Some(message) = self.assembler.merge_fragment(session_id, fragment) {
            let _ = self
                .thread_out
                .send(PacketMessage(session_id, routing_header, message));
        }
    }
}
