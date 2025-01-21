use crate::backend::topology::Topology;
use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::{FloodResponse, NodeType};

impl Topology {
    pub fn add_flood_response(&mut self, flood_id: u64, path: Vec<(NodeId, NodeType)>) {
        if flood_id == self.current_flood_id {
            self.add_path(path);
        }
    }

    pub fn add_path(&mut self, path: Vec<(NodeId, NodeType)>) {
        let hops = path.into_iter().map(|(id, _)| id).collect::<Vec<_>>();

        let Some(last) = hops.last().copied() else {
            return;
        };

        let routing = SourceRoutingHeader::with_first_hop(hops);
        self.leafs.insert(last, routing);
        self.remove_from_waiting(last);
    }

    fn remove_from_waiting(&mut self, destination: NodeId) {
        if let Some(waiting) = self.waiting_packets.remove(&destination) {
            self.waiting_finished_packets.insert(destination, waiting);
        }
    }
}
