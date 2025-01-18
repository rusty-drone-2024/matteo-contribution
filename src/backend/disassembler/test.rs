#![cfg(test)]
use super::Disassembler;
use common_structs::message::Message;
use wg_2024::network::SourceRoutingHeader;

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
    let expected = msg.clone().into_fragments();
    let routing = SourceRoutingHeader::empty_route();
    let fragments = disassembler.split(11, routing, msg);

    assert_eq!(expected, fragments);
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
    let routing = SourceRoutingHeader::empty_route();
    disassembler.add_message_to_send(11, routing, msg);

    for i in 0..fragments.len() {
        assert!(!disassembler.is_message_acked(11));
        assert_eq!(Ok(true), disassembler.ack_fragment(11, i as u64));
    }
    assert!(disassembler.is_message_acked(11));
    assert!(disassembler.remove_message_if_acked(11));
}
