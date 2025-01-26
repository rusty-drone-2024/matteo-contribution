use crate::backend::topology::Topology;
use wg_2024::network::{NodeId};
use wg_2024::packet::NodeType;

impl Topology {
    #[must_use]
    pub fn add_flood_response(
        &mut self,
        flood_id: u64,
        path: Vec<(NodeId, NodeType)>,
    ) -> Option<(NodeId, NodeType)> {
        if flood_id != self.current_flood_id {
            return None;
        }
        self.add_path(path)
    }

    /// Return a new leaf if it is found
    #[must_use]
    pub fn add_path(&mut self, path: Vec<(NodeId, NodeType)>) -> Option<(NodeId, NodeType)> {
        let (last, last_type) = path.last().copied()?;

        let hops = path.into_iter().map(|(id, _)| id).collect::<Vec<_>>();
        for window in hops.windows(2){
            self.graph.add_edge(window[0], window[1], ());
        }
        
        // Only add last as only leaf are valid destination (which are always at end)
        self.remove_from_waiting(last);
        if last_type != NodeType::Drone {
            return Some((last, last_type));
        }
        None
    }

    pub(crate) fn remove_node(&mut self, to_remove: NodeId) {
        self.graph.remove_node(to_remove);
    }
}
