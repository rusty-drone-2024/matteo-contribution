use crate::backend::network::NetworkBackend;
use common_structs::leaf::LeafCommand;
use common_structs::leaf::LeafCommand::{AddSender, Kill, RemoveSender};
use crossbeam_channel::Sender;
use std::collections::HashMap;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

impl NetworkBackend {
    pub(super) fn handle_command(
        neighbors: &mut HashMap<NodeId, Sender<Packet>>,
        command: LeafCommand,
    ) -> bool {
        match command {
            RemoveSender(connection_id) => {
                neighbors.remove(&connection_id);
            }
            AddSender(connection_id, sender) => {
                neighbors.insert(connection_id, sender);
            }
            Kill => {
                return true;
            }
        }

        false
    }
}
