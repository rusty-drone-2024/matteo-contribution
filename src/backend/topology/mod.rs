#![allow(dead_code)]
mod editing;
mod test;
mod traversing;

use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;
use wg_2024::network::NodeId;

pub struct Topology {
    start_id: NodeId,
    current_flood_id: u64,
    graph: DiGraphMap<NodeId, ()>,
    weights: HashMap<NodeId, u8>,
}

impl Topology {}

impl Topology {
    pub fn new(this_node_id: NodeId) -> Self {
        Self {
            start_id: this_node_id,
            current_flood_id: 0,
            graph: DiGraphMap::new(),
            weights: HashMap::default(),
        }
    }

    #[must_use]
    pub fn take_fresh_flood_id(&mut self) -> u64 {
        self.current_flood_id += 1;
        self.current_flood_id
    }
}
