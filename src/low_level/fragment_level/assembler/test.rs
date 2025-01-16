#![cfg(test)]
use super::Assembler;
use common_structs::message::Message;

#[test]
fn missing_session() {
    let mut assembler = Assembler::new();
    assert!(assembler.get_full_message(11).is_none());
}

#[test]
fn assemble_msg() {
    let mut assembler = Assembler::new();

    let msg = Message::RespMedia(
        "Test of somewhat long string passed as media"
            .as_bytes()
            .to_owned(),
    );
    let fragments = msg.clone().into_fragments();

    for f in fragments {
        assert!(
            assembler.get_full_message(11).is_none(),
            "Message shouldn't be ready yet"
        );
        assembler.add_fragment(11, f);
    }
    assert_eq!(Some(msg), assembler.get_full_message(11));
}
