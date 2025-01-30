use crate::backend::network::test::Net;
use common_structs::leaf::LeafCommand::{AddSender, Kill, RemoveSender};
use crossbeam_channel::unbounded;
use std::collections::HashMap;

#[test]
fn test_command_kill() {
    let mut nexts = HashMap::default();

    let exit = Net::handle_command(&mut nexts, Kill);
    assert!(nexts.is_empty(), "Nexts were changed");
    assert!(exit, "Didn't kill");
}

#[test]
fn test_command_add_sender() {
    let mut nexts = HashMap::default();
    let send = unbounded().0;
    let comm = AddSender(10, send.clone());

    let exit = Net::handle_command(&mut nexts, comm);
    assert_eq!(1, nexts.len());
    assert!(nexts.contains_key(&10), "Nexts were not added");
    assert!(
        nexts.get(&10).unwrap().same_channel(&send),
        "Not same channel"
    );
    assert!(!exit, "Kill were shouldn't");
}

#[test]
fn test_command_remove_channel() {
    let send = unbounded().0;
    let mut nexts = [(10, send.clone()), (20, send)].into_iter().collect();
    let comm = RemoveSender(10);

    let exit = Net::handle_command(&mut nexts, comm);
    assert_eq!(1, nexts.len());
    assert!(!nexts.contains_key(&10), "Nexts were not removed");
    assert!(!exit, "Kill were shouldn't");
}
