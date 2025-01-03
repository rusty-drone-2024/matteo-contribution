mod expecting;

use std::collections::HashMap;
use wg_2024::packet::Fragment;

#[allow(dead_code)]
pub struct Assembler {
    expecting_session_ids: HashMap<u64, ToAssemble>,
}

#[derive(Default)]
#[allow(dead_code)]
struct ToAssemble {
    pieces: HashMap<u64, Fragment>,
}
