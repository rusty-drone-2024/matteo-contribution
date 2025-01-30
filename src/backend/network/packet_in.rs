use crate::backend::network::NetworkBackend;
use common_structs::types::{Routing, Session};
use wg_2024::packet::{FloodResponse, Fragment, Nack, NackType, Packet, PacketType};

impl NetworkBackend {
    pub(super) fn handle_packet(&mut self, packet: Packet) {
        let Packet {
            routing_header: routing,
            session_id: session,
            pack_type,
        } = packet;

        match pack_type {
            PacketType::MsgFragment(fragment) => self.handle_fragment(session, fragment, routing),
            PacketType::FloodResponse(resp) => self.handle_flood_resp(resp),
            PacketType::FloodRequest(flood) => {
                let flood_resp = Self::new_flood_resp(self.id, self.node_type, session, flood);
                self.send_packet(flood_resp);
            }
            PacketType::Nack(nack) => {
                let _ = self.handle_nack(session, &nack, &routing);
            }
            PacketType::Ack(ack) => {
                let _ = self.disassembler.ack(session, ack.fragment_index);
            }
        }
    }

    fn handle_flood_resp(&mut self, resp: FloodResponse) {
        let new_leaf = self.topology.add_flood_response(resp);

        if let Some((node_id, node_type)) = new_leaf {
            self.disassembler.ready_sessions_waiting_for(node_id);
            self.send_new_leaf_to_thread(node_id, node_type);
        }
    }

    fn handle_fragment(&mut self, session: Session, fragment: Fragment, routing: Routing) {
        let Some(source) = routing.source() else {
            eprintln!("INTERNAL HOPS EMPTY PRETTY BAD");
            return;
        };

        let rev_hops = routing.get_reversed().hops;
        let _ = self.topology.add_path(&rev_hops, true);

        let ack = Self::new_ack(routing, session, fragment.fragment_index);
        self.send_packet(ack);

        self.send_msg_to_thread(session, source, fragment);
    }

    fn handle_nack(&mut self, session: Session, nack: &Nack, routing: &Routing) -> Option<()> {
        match nack.nack_type {
            NackType::ErrorInRouting(node_id) => {
                self.topology.remove_node(node_id);
            }
            NackType::Dropped => {
                if let Some(source) = routing.source() {
                    self.topology.mark_drop(source);
                }
            }
            _ => {}
        }

        self.disassembler
            .get_mut(session)?
            .wait_for(nack.fragment_index);
        self.send_split(session);
        Some(())
    }
}
