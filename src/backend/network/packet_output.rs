use crate::backend::network::{NetworkBackend, PacketMessage};
use common_structs::leaf::LeafEvent::PacketSend;
use common_structs::types::SessionId;
use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::PacketType::MsgFragment;
use wg_2024::packet::{FloodRequest, Fragment, NodeType, Packet};

impl NetworkBackend {
    pub(super) fn send_message(&mut self, msg: PacketMessage) {
        let PacketMessage {
            session,
            opposite_end: destination,
            message,
        } = msg;
        let fragments = self.disassembler.split(session, destination, message);

        let Some(routing) = self.topology.get_routing_for(destination) else {
            for fragment in fragments {
                self.topology
                    .add_waiting(session, destination, MsgFragment(fragment));
            }
            return;
        };

        for fragment in fragments {
            self.send_packet(Packet::new_fragment(routing.clone(), session, fragment));
        }
    }

    pub(super) fn send_fragment(
        &mut self,
        sesssion: SessionId,
        destination: NodeId,
        fragment: Fragment,
    ) {
        let Some(routing) = self.topology.get_routing_for(destination) else {
            self.topology
                .add_waiting(sesssion, destination, MsgFragment(fragment));
            return;
        };

        self.send_packet(Packet::new_fragment(routing, sesssion, fragment));
    }

    pub(super) fn send_packet(&mut self, packet: Packet) {
        let session_id = packet.session_id;
        let routing = &packet.routing_header;

        let Some(node_id) = routing.current_hop() else {
            eprintln!("DROPPING A PACKET! VERY BAD BEHAVIOUR! PACKET: {packet:?}");
            return;
        };

        let Some(channel) = self.packets_out.get(&node_id) else {
            // TODO Intentional unwrap (as it cannot be empty) -> still remove
            let destination = routing.hops.first().unwrap();
            self.topology
                .add_waiting(session_id, *destination, packet.pack_type);
            return;
        };

        let _ = self.controller_event.send(PacketSend(packet.clone()));
        let _ = channel.send(packet);
    }

    pub(super) fn flood(&self, flood_id: u64) {
        let packet = Packet::new_flood_request(
            SourceRoutingHeader::empty_route(),
            0,
            FloodRequest::initialize(flood_id, self.node_id, NodeType::Client),
        );

        for sender in self.packets_out.values() {
            let _ = sender.send(packet.clone());
        }
    }
}
