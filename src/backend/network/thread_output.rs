use crate::backend::network::NetworkBackend;
use crate::backend::network::NetworkOutput::{MsgReceived, NewLeafFound};
use crate::backend::PacketMessage;
use common_structs::types::SessionId;
use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::{Fragment, NodeType};

impl NetworkBackend {
    pub(super) fn send_msg_to_thread(
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

        let sendable = MsgReceived(PacketMessage::new(session_id, first, message));
        let _ = self.thread_out.send(sendable);
    }

    pub(super) fn send_new_leaf_to_thread(&mut self, node_id: NodeId, node_type: NodeType) {
        let sendable = NewLeafFound(node_id, node_type);
        let _ = self.thread_out.send(sendable);
    }
}
