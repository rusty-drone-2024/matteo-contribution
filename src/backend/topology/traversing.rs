use crate::backend::topology::Topology;
use common_structs::types::Routing;
use petgraph::algo::astar;
use wg_2024::network::NodeId;

impl Topology {
    #[must_use]
    pub fn get_routing_for(&self, to: NodeId) -> Option<Routing> {
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

        Some(Routing::new(path.1, 1))
    }
}
