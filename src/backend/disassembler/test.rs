#![cfg(test)]
use super::Disassembler;
use common_structs::message::Message;

#[test]
fn missing_session() {
    let disassembler = Disassembler::default();
    // TODO better test
    assert!(!disassembler.splits.contains_key(&11));
}

#[test]
fn check_fragment_packets() {
    let mut disassembler = Disassembler::default();

    let msg = Message::RespMedia(
        "Test of somewhat long string passed as media"
            .as_bytes()
            .to_owned(),
    );
    let expected = msg.clone().into_fragments();
    disassembler.split(11, 20, msg);
    let split = disassembler.get(11).unwrap();

    assert_eq!(expected, split.fragments());
}

#[test]
fn check_fragment_acks() {
    let mut disassembler = Disassembler::default();

    let msg = Message::RespMedia(
        "Test of somewhat long string passed as media"
            .as_bytes()
            .to_owned(),
    );
    let fragments = msg.clone().into_fragments();
    disassembler.split(11, 20, msg);

    for i in 0..fragments.len() as u64 {
        assert!(disassembler.splits.contains_key(&11));
        assert_eq!(Ok(true), disassembler.ack(11, i));
    }
    assert!(!disassembler.splits.contains_key(&11));
}
