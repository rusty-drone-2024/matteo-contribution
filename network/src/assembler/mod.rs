mod fragment_adder;
mod merger;
mod test;

use common_structs::types::FragmentIdx;
use std::collections::HashMap;
use wg_2024::packet::Fragment;

/// Is is responsible to group fragments as they arrive
/// and assemble message when they are ready
#[derive(Default)]
pub struct Assembler {
    messages_to_assemble: HashMap<u64, MessageToAssemble>,
}

/// A split message with the pieces already received
struct MessageToAssemble {
    pieces_number: FragmentIdx,
    pieces: HashMap<FragmentIdx, Fragment>,
}

impl MessageToAssemble {
    /// Create a new waiting split message with
    /// the given pieves number.
    fn new(pieces_number: FragmentIdx) -> Self {
        Self {
            pieces_number,
            pieces: HashMap::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentMalformed {}
