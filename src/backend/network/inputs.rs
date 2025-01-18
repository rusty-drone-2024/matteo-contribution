use crate::backend::network::NetworkBackend;
use common_structs::types::SessionId;
use crossbeam_channel::select;
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::PacketType::MsgFragment;
use wg_2024::packet::{FloodRequest, Packet, PacketType};

impl NetworkBackend {
    pub(super) fn read_input_and_chain(&mut self) {
        select! {
            recv(self.packet_in) -> msg => {
                if let Ok(msg) = msg{
                    self.check_packet_and_chain(msg);
                }
            },
            recv(self.thread_in) -> msg => {
                if let Ok(msg) = msg{
                    self.handle_send_msg(msg);
                }
            }
        }
    }

    fn check_packet_and_chain(&mut self, packet: Packet) {
        let Packet {
            session_id,
            routing_header,
            pack_type,
        } = packet;

        // TODO check routing,....

        self.decide_response_and_chain(session_id, routing_header, pack_type);
    }

    fn decide_response_and_chain(
        &mut self,
        session_id: SessionId,
        routing: SourceRoutingHeader,
        pack_type: PacketType,
    ) {
        match pack_type {
            MsgFragment(fragment) => {
                self.handle_send_packet(Packet::new_ack(
                    routing.get_reversed(),
                    session_id,
                    fragment.fragment_index,
                ));
                self.merger_and_chain(session_id, routing, fragment);
            }
            PacketType::Ack(ack) => {
                self.disassembler.ack(session_id, ack.fragment_index);
            }
            PacketType::Nack(nack) => {
                let fragment_to_resend = self
                    .disassembler
                    .nack_require_resend(session_id, nack.fragment_index);

                if let Some((routing, fragment)) = fragment_to_resend {
                    let response = Packet::new_fragment(routing, session_id, fragment);
                    self.handle_send_packet(response);
                }
            }
            PacketType::FloodRequest(flood) => {
                let response = NetworkBackend::packet_response_from_flood_request(flood);
                self.handle_send_packet(response);
            }
            PacketType::FloodResponse(_flood_resp) => {
                todo!() // Create topology
            }
        }
    }

    fn packet_response_from_flood_request(_flood_request: FloodRequest) -> Packet {
        todo!()
    }
}
