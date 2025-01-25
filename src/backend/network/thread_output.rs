use crate::backend::network::{NetworkBackend};
use common_structs::types::SessionId;
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Fragment;
use crate::backend::network::NetworkOutput::MsgReceived;
use crate::backend::PacketMessage;

impl NetworkBackend {
    pub(super) fn send_to_thread(
        &mut self,
        session_id: SessionId,
        routing_header: &SourceRoutingHeader,
        fragment: Fragment,
    ) {
        let Some(message) = self.assembler.merge_fragment(session_id, fragment) else {
            return; // Packet not ready yet
        };

        let Some(first) = routing_header.hops.first().copied() else {
            eprintln!("DRONE PASSED US DATA WITH NO SENSE (EMPTY VEC)");
            return;
        };

        let _ = self
            .thread_out
            .send(MsgReceived(PacketMessage::new(session_id, first, message)));
    }
}
