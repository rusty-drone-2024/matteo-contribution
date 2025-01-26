use petgraph::algo::{astar, has_path_connecting};
use crate::backend::topology::Topology;
use wg_2024::network::{NodeId, SourceRoutingHeader};

impl Topology {
    pub fn get_routing_for(&self, to: NodeId) -> Option<SourceRoutingHeader> {
        let path = astar(&self.graph, self.this_node_id, |finish| finish == to, |_| 1, |_| 0)?;
        
        Some(SourceRoutingHeader::new(path.1, 1))
    }
}
