use crate::backend::network::NetworkBackend;
use common_structs::types::SessionId;
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::PacketType::MsgFragment;
use wg_2024::packet::{FloodRequest, FloodResponse, Nack, NackType, NodeType, Packet, PacketType};

impl NetworkBackend {
    pub(super) fn check_packet_and_chain(&mut self, packet: Packet) {
        let Packet {
            session_id,
            routing_header,
            pack_type,
        } = packet;

        // TODO check routing,....
        //TODO at the moment packet never pass via SC

        self.decide_response_and_chain(session_id, &routing_header, pack_type);
    }

    fn decide_response_and_chain(
        &mut self,
        session_id: SessionId,
        routing: &SourceRoutingHeader,
        pack_type: PacketType,
    ) {
        match pack_type {
            MsgFragment(fragment) => {
                self.send_packet(Packet::new_ack(
                    routing.get_reversed(),
                    session_id,
                    fragment.fragment_index,
                ));
                self.send_to_thread(session_id, routing, fragment);
            }
            PacketType::Ack(ack) => {
                self.disassembler.ack(session_id, ack.fragment_index);
            }
            PacketType::Nack(nack) => {
                self.handle_nack(session_id, &nack);
            }
            PacketType::FloodRequest(flood) => {
                let response = self.create_flood_response_packet(session_id, flood);
                self.send_packet(response);
            }
            PacketType::FloodResponse(flood_resp) => {
                self.topology
                    .add_flood_response(flood_resp.flood_id, flood_resp.path_trace);
            }
        }
    }

    fn handle_nack(&mut self, session_id: SessionId, nack: &Nack) -> Option<()> {
        let (destination, fragment) = self
            .disassembler
            .nack_require_resend(session_id, nack.fragment_index)?;

        if nack.nack_type == NackType::DestinationIsDrone {
            eprintln!("SENT A PACKET TO A DRONE SOMEHOW");
            return None;
        }

        self.send_fragment(session_id, destination, fragment);
        Some(())
    }

    fn create_flood_response_packet(&self, session_id: SessionId, flood: FloodRequest) -> Packet {
        let flood_id = flood.flood_id;
        let mut path_trace = flood.path_trace;

        path_trace.push((self.node_id, NodeType::Client));
        let hops = path_trace.iter().map(|(id, _)| *id).rev().collect();

        Packet::new_flood_response(
            SourceRoutingHeader::with_first_hop(hops),
            session_id,
            FloodResponse {
                flood_id,
                path_trace,
            },
        )
    }
}
