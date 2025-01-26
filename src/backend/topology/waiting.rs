use crate::backend::topology::Topology;
use common_structs::types::SessionId;
use wg_2024::network::NodeId;
use wg_2024::packet::{Packet, PacketType};

impl Topology {
    pub(super) fn remove_from_waiting(&mut self, destination: NodeId) {
        if let Some(waiting) = self.waiting_packets.remove(&destination) {
            self.waiting_finished_packets.insert(destination, waiting);
        }
    }
    
    pub fn add_waiting(
        &mut self,
        sesssion: SessionId,
        destination: NodeId,
        packet_type: PacketType,
    ) {
        self.waiting_packets
            .entry(destination)
            .or_default()
            .push((sesssion, packet_type));

        self.new_waiting += 1;
    }

    #[must_use]
    pub fn require_flood(&mut self) -> bool {
        if self.new_waiting > 0 {
            self.new_waiting = 0;
            return true;
        }
        false
    }

    #[must_use]
    pub fn take_fresh_flood_id(&mut self) -> u64 {
        self.current_flood_id += 1;
        self.current_flood_id
    }

    #[must_use]
    pub fn take_sendable(&mut self) -> Vec<Packet> {
        let mut res = vec![];

        let waiting = self
            .waiting_finished_packets
            .drain()
            .collect::<Vec<(_, Vec<_>)>>();

        for (destination, finished) in waiting {
            for (session_id, content) in finished {
                if let Some(routing_header) = self.get_routing_for(destination) {
                    res.push(Packet {
                        routing_header,
                        session_id,
                        pack_type: content,
                    });
                } else {
                    self.add_waiting(session_id, destination, content);
                }
            }
        }

        res
    }
}
