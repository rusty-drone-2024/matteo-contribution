use crate::backend::network::{NetworkBackend, PacketMessage};
use common_structs::leaf::LeafEvent::{ControllerShortcut, PacketSend};
use common_structs::types::SessionId;
use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::PacketType::MsgFragment;
use wg_2024::packet::{FloodRequest, Fragment, NodeType, Packet};

impl NetworkBackend {
    pub(super) fn send_message(&mut self, msg: PacketMessage) {
        let PacketMessage {
            session,
            opposite_end: dest,
            message,
        } = msg;
        println!("SENDING MESSAGE from {} to {dest} of type {}", self.id, &message);
        self.disassembler.split(session, dest, message);
        self.send_split(session);
    }

    /// # return
    /// True if send immediately, false else and if there was a problem
    pub(super) fn send_split(&mut self, session: SessionId) -> Option<bool> {
        let split = self.disassembler.get_mut(session)?;
        let dest = split.destination();

        let Some(routing) = self.topology.get_routing_for(dest) else {
            split.add_all_to_waiting();
            return Some(false);
        };

        for fragment in split.take_waiting() {
            self.send_packet(Packet::new_fragment(routing.clone(), session, fragment));
        }

        Some(true)
    }

    pub(super) fn send_fragment(
        &mut self,
        sesssion: SessionId,
        destination: NodeId,
        fragment: Fragment,
    ) {
        let Some(routing) = self.topology.get_routing_for(destination) else {
            let _ = self
                .disassembler
                .add_waiting(sesssion, destination, fragment.fragment_index);
            return;
        };

        self.send_packet(Packet::new_fragment(routing, sesssion, fragment));
    }

    pub(super) fn send_packet(&mut self, packet: Packet) {
        let session = packet.session_id;
        let routing = &packet.routing_header;

        let Some(destination) = routing.hops.last() else {
            return eprintln!(
                "DROPPING A PACKET AS ROUTING IS EMPTY! VERY BAD! PACKET: {packet:?}"
            );
        };

        let Some(node_id) = routing.current_hop() else {
            return eprintln!("DROPPING A PACKET AS NO NEXT HOP! VERY BAD! PACKET: {packet:?}");
        };

        let Some(channel) = self.packets_out.get(&node_id) else {
            match packet.pack_type {
                MsgFragment(fragment) => {
                    let _ = self.disassembler.add_waiting(
                        session,
                        *destination,
                        fragment.fragment_index,
                    );
                }
                _ => {
                    self.shortcut(packet);
                }
            }
            return;
        };

        let _ = self.controller_event.send(PacketSend(packet.clone()));
        let _ = channel.send(packet);
    }

    pub(super) fn flood(&mut self) {
        let flood_id = self.topology.take_fresh_flood_id();
        println!("==> FLOODING FROM {}", self.id);

        let packet = Packet::new_flood_request(
            SourceRoutingHeader::empty_route(),
            0,
            FloodRequest::initialize(flood_id, self.id, NodeType::Client),
        );

        for sender in self.packets_out.values() {
            let _ = sender.send(packet.clone());
        }
    }

    pub(super) fn shortcut(&self, packet: Packet) {
        let _ = self.controller_event.send(ControllerShortcut(packet));
    }
}
