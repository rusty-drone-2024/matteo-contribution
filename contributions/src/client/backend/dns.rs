use common_structs::message::{Link, ServerUUID};
use std::collections::HashMap;
use wg_2024::network::NodeId;

#[derive(Default)]
pub struct Dns {
    servers: HashMap<ServerUUID, NodeId>,
    file_to_uuid: HashMap<Link, ServerUUID>,
}

impl Dns {
    pub(super) fn add_server(&mut self, uuid: ServerUUID, node_id: NodeId) {
        self.servers.insert(uuid, node_id);
    }

    pub(super) fn save(&mut self, link: Link, uuid: ServerUUID) {
        self.file_to_uuid.insert(link, uuid);
    }

    pub(super) fn get(&self, link: &Link) -> Option<NodeId> {
        let uuid = self.file_to_uuid.get(link)?;
        self.servers.get(uuid).copied()
    }

    pub(crate) fn get_server_uuid(&self, node_id: NodeId) -> Option<ServerUUID> {
        // TODO in favor of safer ways (so no unwap)
        self.servers.iter().find_map(|(uuid, id)| {
            if *id == node_id {
                return Some(*uuid);
            }
            None
        })
    }
}
