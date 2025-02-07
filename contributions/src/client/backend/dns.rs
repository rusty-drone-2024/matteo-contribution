use common_structs::message::{Link, ServerUUID};
use std::collections::HashMap;
use wg_2024::network::NodeId;

/// Model a connection between
/// `Link` -> `ServerUUID` -> `NodeId`
/// Each `Link` can be connected to a single `ServerUUID`
/// and each `ServerUUID` can be connected to a single `NodeId`
/// This assume two things:
/// 1. if a server has the same `ServerUUID` then it has the same files.
/// 2. if a two files have the same `Link` they are identical.
#[derive(Default)]
pub struct Dns {
    servers: HashMap<ServerUUID, NodeId>,
    file_to_uuid: HashMap<Link, ServerUUID>,
}

impl Dns {
    /// Add a link between `uuid` and `node_id`
    /// it replace the previous `uuid` association.
    pub fn add_server(&mut self, uuid: ServerUUID, node_id: NodeId) {
        self.servers.insert(uuid, node_id);
    }

    /// Add a link between `link` and `uuid`
    /// it replace the previous `link` association.
    pub fn save(&mut self, link: Link, uuid: ServerUUID) {
        self.file_to_uuid.insert(link, uuid);
    }

    /// Get the `NodeId` associated to a `link`
    /// # Return
    /// The `NodeId` associated or Node if any step of linking is missing.
    pub fn get(&self, link: &Link) -> Option<NodeId> {
        let uuid = self.file_to_uuid.get(link)?;
        self.servers.get(uuid).copied()
    }
}
