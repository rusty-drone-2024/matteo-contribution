use crate::backend::network::NetworkBackend;
use common_structs::types::SessionId;
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::NackType::{ErrorInRouting, UnexpectedRecipient};
use wg_2024::packet::{FloodRequest, FloodResponse, Nack, NackType, Packet, PacketType};

impl NetworkBackend {
    pub(super) fn check_packet_and_chain(&mut self, packet: Packet) {
        let Packet {
            session_id: session,
            routing_header: routing,
            pack_type,
        } = packet;

        if let PacketType::FloodRequest(_) = pack_type {
            return self.decide_response_and_chain(session, &routing, pack_type);
        }

        if Some(self.id) != routing.current_hop() {
            let nack_type = UnexpectedRecipient(self.id);
            return if let PacketType::MsgFragment(fragment) = &pack_type {
                self.nack(routing, session, fragment.fragment_index, nack_type);
            } else {
                self.shortcut(Packet {
                    session_id: session,
                    routing_header: routing,
                    pack_type,
                });
            };
        }

        if let Some(next) = routing.next_hop() {
            return if let PacketType::MsgFragment(fragment) = &pack_type {
                self.nack(
                    routing,
                    session,
                    fragment.fragment_index,
                    ErrorInRouting(next),
                );
            } else {
                self.shortcut(Packet {
                    session_id: session,
                    routing_header: routing,
                    pack_type,
                });
            };
        }
        self.decide_response_and_chain(session, &routing, pack_type);
    }

    fn decide_response_and_chain(
        &mut self,
        session: SessionId,
        routing: &SourceRoutingHeader,
        pack_type: PacketType,
    ) {
        match pack_type {
            PacketType::MsgFragment(fragment) => {
                let _ = self.topology.add_path(&routing.hops, true);
                self.ack(routing.clone(), session, fragment.fragment_index);
                self.send_msg_to_thread(session, routing, fragment);
            }
            PacketType::Ack(ack) => {
                self.disassembler.ack(session, ack.fragment_index).ok();
            }
            PacketType::Nack(nack) => {
                self.handle_nack(session, &nack, routing);
            }
            PacketType::FloodRequest(flood) => {
                let response = self.create_flood_response_packet(session, flood);
                self.send_packet(response);
            }
            PacketType::FloodResponse(flood_resp) => {
                let new_leaf = self
                    .topology
                    .add_flood_response(flood_resp.flood_id, flood_resp.path_trace);

                if let Some((node_id, node_type)) = new_leaf {
                    self.disassembler.remove_waiting_for(node_id);
                    self.send_new_leaf_to_thread(node_id, node_type);
                }
            }
        }
    }

    fn handle_nack(
        &mut self,
        session: SessionId,
        nack: &Nack,
        routing: &SourceRoutingHeader,
    ) -> Option<()> {
        let split = self.disassembler.get(session)?;
        let fragment = split.get_fragment(nack.fragment_index).ok()?;
        let dest = split.destination();

        match nack.nack_type {
            ErrorInRouting(node_id) => {
                self.topology.remove_node(node_id);
            }
            NackType::Dropped => {
                if let Some(first) = routing.hops.first() {
                    self.topology.mark_drop(*first);
                }
            }
            NackType::DestinationIsDrone => {
                eprintln!("SENT A PACKET TO A DRONE SOMEHOW");
            }
            UnexpectedRecipient(dest) => {
                eprintln!("SENT A PACKET TO {dest} SOMEHOW");
            }
        }

        if nack.nack_type == NackType::DestinationIsDrone {
            return None;
        }

        self.send_fragment(session, dest, fragment);
        Some(())
    }

    fn create_flood_response_packet(&self, session: SessionId, flood: FloodRequest) -> Packet {
        let flood_id = flood.flood_id;
        let mut path_trace = flood.path_trace;

        path_trace.push((self.id, self.node_type));
        let hops = path_trace.iter().map(|(id, _)| *id).rev().collect();

        Packet::new_flood_response(
            SourceRoutingHeader::with_first_hop(hops),
            session,
            FloodResponse {
                flood_id,
                path_trace,
            },
        )
    }
}
