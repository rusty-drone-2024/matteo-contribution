use crate::topology::Topology;
use common_structs::types::Routing;
use petgraph::algo::astar;
use wg_2024::network::NodeId;

impl Topology {
    /// Get a routing for a particular node `to` and from the starting id.
    /// It try to minimize the probability that the packet will be dropped by choosing
    /// the path that has the minimum sum of pdr estimation for each nodes.
    /// The path is also aged, the pdr estimation is decreased. For this reason it
    /// is assumed by the function that the routing in question will be used.
    /// # Return
    /// The routing if one is find. Else in case of the two node are not connected it
    /// returns `None`.
    // TODO use probability estimation instead
    #[must_use]
    pub fn get_routing_for(&mut self, to: NodeId) -> Option<Routing> {
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
        )?
        .1;

        self.age_path(&path);

        Some(Routing::new(path, 1))
    }

    /// Age the path, the pdr estimation is decreased for each of the node
    /// in the chain clamped at 0.
    //TODO Test expecially if it is too strong (or probably too weak)
    pub(super) fn age_path(&mut self, path: &[NodeId]) {
        for node in path {
            self.update_weight(*node, true);
        }
    }
}
