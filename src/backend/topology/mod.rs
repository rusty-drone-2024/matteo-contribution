#![allow(dead_code)]
mod editing;
mod traversing;
mod waiting;

use common_structs::types::SessionId;
use std::collections::HashMap;
use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::PacketType;

pub struct Topology {
    this_node_id: NodeId,
    current_flood_id: u64,
    // TODO Replace with PetGraph
    leafs: HashMap<NodeId, SourceRoutingHeader>,
    waiting_packets: HashMap<NodeId, Vec<(SessionId, PacketType)>>,
    waiting_finished_packets: HashMap<NodeId, Vec<(SessionId, PacketType)>>,
    new_waiting: usize,
}

impl Topology {}

impl Topology {
    pub fn new(this_node_id: NodeId) -> Self {
        Self {
            this_node_id,
            current_flood_id: 0,
            leafs: HashMap::default(),
            waiting_packets: HashMap::default(),
            waiting_finished_packets: HashMap::default(),
            new_waiting: 0,
        }
    }
}
