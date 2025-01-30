use crate::backend::network::NetworkBackend;
use crate::backend::network::NetworkOutput::{MsgReceived, NewLeafFound};
use crate::backend::PacketMessage;
use common_structs::types::Session;
use wg_2024::network::NodeId;
use wg_2024::packet::{Fragment, NodeType};

impl NetworkBackend {
    pub(super) fn send_msg_to_thread(
        &mut self,
        session: Session,
        source: NodeId,
        fragment: Fragment,
    ) {
        let Some(message) = self.assembler.merge_fragment(session, fragment) else {
            return; // Packet not ready yet
        };

        println!("<==RECEIVED=== ({} <- {}): {}", self.id, source, message);
        let sendable = MsgReceived(PacketMessage::new(session, source, message));
        let _ = self.thread_out.send(sendable);
    }

    pub(super) fn send_new_leaf_to_thread(&mut self, node_id: NodeId, node_type: NodeType) {
        let sendable = NewLeafFound(node_id, node_type);
        let _ = self.thread_out.send(sendable);
    }
}
