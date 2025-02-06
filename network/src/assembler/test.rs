#![cfg(test)]
use super::Assembler;
use common_structs::message::Message;

#[test]
fn missing_session() {
    let mut assembler = Assembler::default();
    assert!(assembler.take_full_message(11).is_none());
}

#[test]
fn assemble_msg() {
    let mut assembler = Assembler::default();

    let msg = Message::RespMedia(
        "Test of somewhat long string passed as media"
            .as_bytes()
            .to_owned(),
    );
    let fragments = msg.clone().into_fragments();

    for f in fragments {
        assert!(
            assembler.take_full_message(11).is_none(),
            "Message shouldn't be ready yet"
        );
        assert_eq!(Ok(true), assembler.merge_fragment(11, f));
    }
    assert_eq!(Some(msg), assembler.take_full_message(11));
}
