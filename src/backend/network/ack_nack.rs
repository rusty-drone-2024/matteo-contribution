use common_structs::types::{FragmentIndex, SessionId};
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::{Nack, NackType, Packet};
use crate::backend::network::NetworkBackend;

impl NetworkBackend{
    pub(super) fn ack(&mut self, mut routing: SourceRoutingHeader, session_id: SessionId, fragment_index: FragmentIndex){
        routing.reverse();
        routing.increase_hop_index();
        self.send_packet(Packet::new_ack(
            routing,
            session_id,
            fragment_index,
        ));
    }

    pub(super) fn nack(&mut self, mut routing: SourceRoutingHeader, session_id: SessionId, fragment_index: FragmentIndex, nack_type: NackType) {
        routing.reverse();
        routing.increase_hop_index();
        self.send_packet(Packet::new_nack(
            routing,
            session_id,
            Nack { fragment_index, nack_type },
        ));
    }
}