use crate::backend::NetworkBackend;
use wg_2024::network::NodeId;
use wg_2024::packet::NackType::{ErrorInRouting, UnexpectedRecipient};
use wg_2024::packet::PacketType::{FloodRequest, MsgFragment};
use wg_2024::packet::{NackType, Packet};

impl NetworkBackend {
    #[must_use]
    pub(super) fn find_routing_error(self_id: NodeId, packet: &Packet) -> Option<NackType> {
        if let FloodRequest(_) = &packet.pack_type {
            return None;
        }

        let routing = &packet.routing_header;

        if Some(self_id) != routing.current_hop() {
            return Some(UnexpectedRecipient(self_id));
        } else if let Some(next) = routing.next_hop() {
            return Some(ErrorInRouting(next));
        }

        None
    }

    pub(super) fn handle_error(&mut self, packet: Packet, nack_type: NackType) {
        let MsgFragment(fragment) = &packet.pack_type else {
            return self.shortcut(packet);
        };

        self.send_packet(Self::new_nack(
            self.id,
            packet.routing_header,
            packet.session_id,
            fragment.fragment_index,
            nack_type,
        ));
    }
}
