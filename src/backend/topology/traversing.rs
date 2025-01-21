use crate::backend::topology::Topology;
use wg_2024::network::{NodeId, SourceRoutingHeader};

impl Topology {
    pub fn get_routing_for(&self, node_id: NodeId) -> Option<SourceRoutingHeader> {
        self.leafs.get(&node_id).cloned()
    }
}
