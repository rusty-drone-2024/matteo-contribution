#![cfg(test)]
use super::Disassembler;
use common_structs::message::Message;

#[test]
fn missing_session() {
    let disassembler = Disassembler::default();
    assert!(!disassembler.splits.contains_key(&11));
    assert!(disassembler.get(11).is_none());
}

#[test]
fn fragment_packets() {
    let mut disassembler = Disassembler::default();

    let msg = Message::RespMedia(
        "Test of somewhat long string passed as media"
            .as_bytes()
            .to_owned(),
    );
    let expected = msg.clone().into_fragments();
    disassembler.split(11, 20, msg);
    let split = disassembler.get(11).unwrap();

    let mut frag = split.fragments();
    frag.sort_by(|a, b| a.fragment_index.cmp(&b.fragment_index));
    assert_eq!(expected, frag);
}

#[test]
fn fragment_acks() {
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
        let is_last = i == (fragments.len() - 1) as u64;
        assert_eq!(Ok(is_last), disassembler.ack(11, i));
    }
    assert!(!disassembler.splits.contains_key(&11));
}

#[test]
fn flood_not_needed() {
    let mut disassembler = Disassembler::default();
    assert!(!disassembler.require_flood(false));
}

#[test]
fn flood_needed() {
    let mut disassembler = Disassembler::default();
    disassembler.split(0, 0, Message::ReqServerType);
    let _ = disassembler.add_session_to_wait_queue(0);
    assert!(disassembler.require_flood(false));
}

#[test]
fn flood_cleared() {
    let mut disassembler = Disassembler::default();
    disassembler.split(0, 0, Message::ReqServerType);
    let _ = disassembler.add_session_to_wait_queue(0);
    assert!(disassembler.require_flood(false));
    assert!(!disassembler.require_flood(false));
}

#[test]
fn split_add_all_to_waiting() {
    let mut disassembler = Disassembler::default();
    disassembler.split(0, 0, Message::ReqServerType);

    disassembler.add_session_to_wait_queue(0).unwrap();
    assert!(disassembler.waiting.contains_key(&0));
}

#[test]
fn get_and_get_mut_fail() {
    let mut disassembler = Disassembler::default();
    assert!(disassembler.get(0).is_none());
    assert!(disassembler.get_mut(0).is_none());
}

#[test]
fn get_and_get_mut_pass() {
    let mut disassembler = Disassembler::default();
    disassembler.split(0, 0, Message::ReqFilesList);
    assert!(disassembler.get(0).is_some());
    assert!(disassembler.get_mut(0).is_some());
}

#[test]
fn add_waiting_to_nothing() {
    let mut disassembler = Disassembler::default();
    assert!(disassembler.add_session_to_wait_queue(0).is_err());
}

#[test]
fn add_waiting_to_already_waiting_all() {
    let mut disassembler = Disassembler::default();
    disassembler.split(0, 0, Message::RespMedia(vec![0; 100]));
    assert!(disassembler.add_session_to_wait_queue(0).is_ok());
}

#[test]
fn take_ready_empty() {
    let mut disassembler = Disassembler::default();
    disassembler.split(0, 0, Message::ReqServerType);
    assert_eq!(1, disassembler.get_mut(0).unwrap().take_to_send().len());
    assert!(disassembler.take_ready_session().is_empty());
}

#[test]
fn take_ready_not_empty() {
    let mut disassembler = Disassembler::default();
    disassembler.split(10, 1, Message::ReqServerType);
    disassembler.split(20, 2, Message::ReqServerType);

    disassembler.add_session_to_wait_queue(10).unwrap();

    disassembler.add_session_to_wait_queue(20).unwrap();

    disassembler.ready_sessions_waiting_for(1);
    assert_eq!(vec![10], disassembler.take_ready_session());
}
