mod fragment_adder;
mod merger;
mod test;

use std::collections::HashMap;
use wg_2024::packet::Fragment;

#[derive(Default)]
pub struct Assembler {
    messages_to_assemble: HashMap<u64, MessageToAssemble>,
}

struct MessageToAssemble {
    pieces_number: u64,
    pieces: HashMap<u64, Fragment>,
}

impl MessageToAssemble {
    fn new(pieces_number: u64) -> Self {
        Self {
            pieces_number,
            pieces: HashMap::default(),
        }
    }
}
