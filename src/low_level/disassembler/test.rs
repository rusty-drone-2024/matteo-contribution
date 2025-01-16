#![cfg(test)]
use crate::low_level::disassembler::Disassembler;
use common_structs::message::Message;
use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::Packet;

#[test]
fn missing_session() {
    let disassembler = Disassembler::new();
    assert!(!disassembler.is_message_acked(11));
}

#[test]
fn check_fragment_packets() {
    let mut disassembler = Disassembler::new();

    let msg = Message::RespMedia(
        "Test of somewhat long string passed as media"
            .as_bytes()
            .to_owned(),
    );
    let fragments = msg.clone().into_fragments();
    disassembler.add_message_to_send(11, msg);

    for (i, f) in fragments.into_iter().enumerate() {
        let routing = SourceRoutingHeader::empty_route();
        let expected = Some(Packet::new_fragment(routing.clone(), 11, f));
        assert_eq!(
            expected,
            disassembler.get_packet_for_fragment(routing.clone(), 11, i as u64)
        );
    }
}

#[test]
fn check_fragment_acks() {
    let mut disassembler = Disassembler::new();

    let msg = Message::RespMedia(
        "Test of somewhat long string passed as media"
            .as_bytes()
            .to_owned(),
    );
    let fragments = msg.clone().into_fragments();
    disassembler.add_message_to_send(11, msg);

    for i in 0..fragments.len() {
        assert!(!disassembler.is_message_acked(11));
        assert_eq!(Ok(true), disassembler.ack_fragment(11, i as u64));
    }
    assert!(disassembler.is_message_acked(11));
    assert!(disassembler.remove_acked_message(11));
}
