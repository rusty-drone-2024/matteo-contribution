#![allow(dead_code)]
mod fragment_adder;
mod merger;
mod test;

use std::collections::HashMap;
use wg_2024::packet::Fragment;

pub struct Assembler {
    messages_to_assemble: HashMap<u64, MessageToAssemble>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            messages_to_assemble: HashMap::new(),
        }
    }
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
