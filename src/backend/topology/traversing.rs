use crate::backend::topology::Topology;
use petgraph::algo::astar;
use wg_2024::network::{NodeId, SourceRoutingHeader};

impl Topology {
    #[must_use]
    pub fn get_routing_for(&self, to: NodeId) -> Option<SourceRoutingHeader> {
        let edge_cost = |(node_start, _, &())| {
            let weight = self.weights.get(&node_start).copied().unwrap_or_default();
            u64::from(weight) + 1
        };

        let path = astar(
            &self.graph,
            self.start_id,
            |finish| finish == to,
            edge_cost,
            |_| 0,
        )?;

        Some(SourceRoutingHeader::new(path.1, 1))
    }
}
