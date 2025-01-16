use crate::low_level::network_level::NetworkHandler;
use wg_2024::network::{NodeId, SourceRoutingHeader};

impl NetworkHandler {
    pub(crate) fn get_valid_server_id(&self) -> Vec<NodeId> {
        //TODO actual
        vec![20]
    }

    pub(crate) fn get_routing_to(&self, node_id: NodeId) -> Option<SourceRoutingHeader> {
        //TODO actual
        if node_id != 20 {
            return None;
        }

        Some(SourceRoutingHeader::initialize(vec![10, 20]))
    }
}
