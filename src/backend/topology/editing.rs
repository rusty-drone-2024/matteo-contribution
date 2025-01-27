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

        let last = flood_path.last().copied()?;
        let path = flood_path.into_iter().map(|(id, _)| id).collect::<Vec<_>>();
        self.add_path(&path);

        // Only add last as only leaf are valid destination (which are always at end)
        if last.1 == NodeType::Drone {
            return None;
        }
        Some(last)
    }

    pub fn add_path(&mut self, path: &[NodeId]) {
        for window in path.windows(2) {
            self.graph.add_edge(window[0], window[1], ());
        }
    }

    pub fn remove_node(&mut self, to_remove: NodeId) {
        self.graph.remove_node(to_remove);
    }
}
