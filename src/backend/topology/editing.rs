use crate::backend::topology::Topology;
use wg_2024::network::NodeId;
use wg_2024::packet::NodeType;

impl Topology {
    #[must_use]
    /// Return a new leaf if it is found
    pub fn add_flood_response(
        &mut self,
        flood_id: u64,
        flood_path: Vec<(NodeId, NodeType)>,
    ) -> Option<(NodeId, NodeType)> {
        if flood_id != self.current_flood_id {
            return None;
        }

        let (id, node_type) = flood_path.last().copied()?;
        let path = flood_path.into_iter().map(|(id, _)| id).collect::<Vec<_>>();

        // Only add last as only leaf are valid destination (which are always at end)
        if node_type == NodeType::Drone {
            let _ = self.add_path(&path, false);
            return None;
        }

        let _ = self.add_path(&path, true);
        Some((id, node_type))
    }

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

    pub fn remove_node(&mut self, to_remove: NodeId) {
        self.graph.remove_node(to_remove);
    }

    pub fn mark_drop(&mut self, to_mark: NodeId) {
        self.update_weight(to_mark, false);
    }

    pub(super) fn update_weight(&mut self, node: NodeId, positive: bool) {
        const MEM_LEN: u64 = 50;
        const NEG_FACTOR: u64 = 200;

        let new_res = u64::from(!positive) * NEG_FACTOR;

        let weight = self.weights.entry(node).or_default();
        let new_weight = (u64::from(*weight) * (MEM_LEN - 1) + new_res) / MEM_LEN;

        *weight = u8::try_from(new_weight).unwrap_or(0);
    }
}
