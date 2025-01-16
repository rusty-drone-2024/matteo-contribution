use super::{Assembler, MessageToAssemble};
use common_structs::types::SessionId;
use wg_2024::packet::Fragment;

impl Assembler {
    pub fn add_fragment(&mut self, session_id: SessionId, fragment: Fragment) {
        let current_len = fragment.total_n_fragments;
        let to_assemble = self
            .messages_to_assemble
            .entry(session_id)
            .or_insert_with(|| MessageToAssemble::new(current_len));

        assert!(fragment.fragment_index < fragment.total_n_fragments);
        to_assemble.pieces.insert(fragment.fragment_index, fragment);
        assert_eq!(current_len, to_assemble.pieces_number);
    }
}
