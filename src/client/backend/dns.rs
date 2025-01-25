use crate::client::backend::ClientBackend;
use common_structs::message::Link;
use wg_2024::network::NodeId;

impl ClientBackend {
    pub(super) fn save_to_dns(&mut self, server_id: NodeId, link: Link) {
        self.dns.insert(link, server_id);
    }

    pub(super) fn get_from_dns(&self, link: &Link) -> Option<NodeId> {
        self.dns.get(link).copied()
    }
}
