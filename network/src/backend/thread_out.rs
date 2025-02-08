use crate::backend::NetworkBackend;
use crate::backend::NetworkOutput::{MsgReceived, NewLeafFound};
use crate::PacketMessage;
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
        let _ = self.assembler.merge_fragment(session, fragment);

        let Some(message) = self.assembler.take_full_message(session) else {
            return; // Packet not ready yet
        };

        println!("\t===RECEIVED==> ({} <- {}): {}", self.id, source, message);
        let sendable = MsgReceived(PacketMessage::new(session, source, message));
        let _ = self.thread_out.send(sendable);
    }

    pub(super) fn send_new_leaf_to_thread(&self, node_id: NodeId, node_type: NodeType) {
        let sendable = NewLeafFound(node_id, node_type);
        let _ = self.thread_out.send(sendable);
    }
}
