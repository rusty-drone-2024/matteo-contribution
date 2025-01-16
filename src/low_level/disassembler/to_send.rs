use super::{DisassembledPacket, Disassembler};
use common_structs::message::Message;
use common_structs::types::{FragmentIndex, SessionId};
use std::collections::HashSet;
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Packet;

impl Disassembler {
    pub fn add_message_to_send(&mut self, session_id: SessionId, message: Message) -> bool {
        let disassembled = DisassembledPacket {
            pieces: message.into_fragments(),
            ack_received: HashSet::default(),
        };

        self.packets_to_send
            .insert(session_id, disassembled)
            .is_some()
    }

    pub fn get_packet_for_fragment(
        &self,
        routing_header: SourceRoutingHeader,
        session_id: SessionId,
        fragment_id: FragmentIndex,
    ) -> Option<Packet> {
        let disassembled = self.packets_to_send.get(&session_id)?;
        let fragment = disassembled.pieces.get(fragment_id as usize)?;

        Some(Packet::new_fragment(
            routing_header,
            session_id,
            fragment.clone(),
        ))
    }

    pub fn get_fragment_size(&self, session_id: SessionId) -> Option<u64> {
        let disassembled = self.packets_to_send.get(&session_id)?;
        Some(disassembled.pieces.len() as u64)
    }
}
