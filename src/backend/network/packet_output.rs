use crate::backend::network::{NetworkBacked, PacketMessage};
use wg_2024::packet::Packet;

impl NetworkBacked {
    pub(super) fn handle_send_msg(&mut self, msg: PacketMessage) {
        let PacketMessage(session, routing, message) = msg;
        let fragments = self.disassembler.split(session, message);

        for fragment in fragments {
            let packet = Packet::new_fragment(routing.clone(), session, fragment);
            self.handle_send_packet(packet);
        }
    }

    pub(super) fn handle_send_packet(&mut self, packet: Packet) {
        let _ = self.packet_out.send(packet);
    }
}
