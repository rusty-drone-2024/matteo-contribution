use super::{Assembler, MessageToAssemble};
use common_structs::types::SessionId;
use wg_2024::packet::Fragment;

impl Assembler {
    pub(crate) fn add_fragment(&mut self, session_id: SessionId, fragment: Fragment) -> bool {
        let total = fragment.total_n_fragments;

        let to_assemble = self
            .messages_to_assemble
            .entry(session_id)
            .or_insert_with(|| MessageToAssemble::new(total));

        if Assembler::is_valid_fragment_index(
            fragment.fragment_index,
            total,
            to_assemble.pieces_number,
        ) {
            return to_assemble
                .pieces
                .insert(fragment.fragment_index, fragment)
                .is_none();
        }
        false
    }

    pub(crate) fn is_valid_fragment_index(index: u64, total: u64, pieces_number: u64) -> bool {
        index < total && total == pieces_number
    }
}
