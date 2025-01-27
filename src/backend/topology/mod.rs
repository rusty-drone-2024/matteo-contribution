#![allow(dead_code)]
mod editing;
mod test;
mod traversing;

use petgraph::graphmap::UnGraphMap;
use wg_2024::network::NodeId;

pub struct Topology {
    start_id: NodeId,
    current_flood_id: u64,
    graph: UnGraphMap<u8, ()>,
}

impl Topology {}

impl Topology {
    pub fn new(this_node_id: NodeId) -> Self {
        Self {
            start_id: this_node_id,
            current_flood_id: 0,
            graph: UnGraphMap::new(),
        }
    }

    #[must_use]
    pub fn take_fresh_flood_id(&mut self) -> u64 {
        self.current_flood_id += 1;
        self.current_flood_id
    }
}
