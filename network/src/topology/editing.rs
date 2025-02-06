use crate::topology::Topology;
use wg_2024::network::NodeId;
use wg_2024::packet::{FloodResponse, NodeType};

impl Topology {
    /// Add the information contained in a `FloodResponse` to the topology,
    /// only if it has the current `flood_id` else it ignores it,
    /// adjusting the understanding by adding the nodes and edges required.
    /// The path must be a valid path starting with this node id.
    /// # Return
    /// A new leaf that is added to the topology. It returns its `NodeId` and `NodeType`.
    /// Only a single leaf can be return at max as no path can pass through a client.
    #[must_use]
    pub fn add_flood_response(&mut self, flood_res: FloodResponse) -> Option<(NodeId, NodeType)> {
        let FloodResponse { flood_id, path_trace }= flood_res;
        if flood_id != self.current_flood_id {
            return None;
        }

        let (id, node_type) = path_trace.last().copied()?;
        let path = path_trace.into_iter().map(|(id, _)| id).collect::<Vec<_>>();

        if node_type == NodeType::Drone {
            let _ = self.add_path(&path, false);
            return None;
        }

        let _ = self.add_path(&path, true);
        Some((id, node_type))
    }

    /// Add the information contained in the `path` to the topology,
    /// adjusting the understanding by adding the nodes and edges required.
    /// The path must be a valid path starting with this node id.
    /// # Return
    /// A new leaf that is added to the topology. It returns its `NodeId` and `NodeType`.
    /// Only a single leaf can be return at max as no path can pass through a client.
    pub fn add_path(&mut self, path: &[NodeId], is_last_leaf: bool) -> Result<(), String> {
        if Some(self.start_id) != path.first().copied() {
            return Err("Path does not start with this node".to_string());
        }

        let windows = path.windows(2);
        let last_index = windows.len() - 1;

        for (i, window) in windows.enumerate() {
            let a = window[0];
            let b = window[1];

            self.graph.add_edge(a, b, ());
            if i != 0 && !(is_last_leaf && i == last_index) {
                self.graph.add_edge(b, a, ());
            }
        }

        Ok(())
    }

    /// Removes a node from the topology if it exists
    pub fn remove_node(&mut self, to_remove: NodeId) {
        self.graph.remove_node(to_remove);
    }

    /// Mark a node that as dropped a fragment by increasing its
    /// pdr estimation (its `Weight`) to make it less likely to be picked again.
    pub fn mark_drop(&mut self, to_mark: NodeId) {
        self.update_weight(to_mark, false);
    }

    /// Update a `Weight` of a node by increasing or decreasing based on the
    /// `positive` flag. If this flag is set the `weight` is decreased in order
    /// to make it more likely to be picked in the future.
    pub(super) fn update_weight(&mut self, node: NodeId, positive: bool) {
        const MEM_LEN: u64 = 50;
        const NEG_FACTOR: u64 = 200;

        let new_res = u64::from(!positive) * NEG_FACTOR;

        let weight = self.weights.entry(node).or_default();
        let new_weight = (u64::from(*weight) * (MEM_LEN - 1) + new_res) / MEM_LEN;

        *weight = u8::try_from(new_weight).unwrap_or(0);
    }
}
