#![allow(dead_code)]
mod editing;
mod test;
mod traversing;

use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;
use wg_2024::network::NodeId;

/// The weight of a single node, is an estimation of
/// the drone pdr. The higher the number the higher the pdr is.
pub type Weight = u8;

/// Represent the topology using a directed graph
/// to avoid passing thought the client and use
/// weight to estimate the drop rate.
/// Also keep track of `flood_id`.
pub struct Topology {
    start_id: NodeId,
    current_flood_id: u64,
    graph: DiGraphMap<NodeId, ()>,
    weights: HashMap<NodeId, u8>,
}

impl Topology {
    /// Create a new topology with the given id
    /// that will be used as a starting point for all flood and
    /// reachability information.
    pub fn new(this_node_id: NodeId) -> Self {
        Self {
            start_id: this_node_id,
            current_flood_id: 0,
            graph: DiGraphMap::new(),
            weights: HashMap::default(),
        }
    }

    /// Get a new fresh `flood_id`. Doing so invalidate the old one making
    /// it so that all `FloodResponse` received after this call that have 
    /// older `flood_id` will be ignored.
    #[must_use]
    pub fn take_fresh_flood_id(&mut self) -> u64 {
        self.current_flood_id += 1;
        self.current_flood_id
    }
}
