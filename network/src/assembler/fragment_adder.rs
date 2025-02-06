use super::{Assembler, FragmentMalformed, MessageToAssemble};
use common_structs::types::{FragmentIdx, Session};
use wg_2024::packet::Fragment;

impl Assembler {
    /// Add the `fragment` to the pile that need to be assembled.
    /// If the group with the given `session_id` doesn't exist than
    /// it is created.
    /// # Return
    /// In case of success it return wheter the fragment was cleanly inserted
    /// or wheter it is was already received.
    /// # Errors
    /// If the message was not correctly fragmented on the other side.
    pub fn merge_fragment(
        &mut self,
        session: Session,
        fragment: Fragment,
    ) -> Result<bool, FragmentMalformed> {
        let total = fragment.total_n_fragments;

        let to_assemble = self
            .messages_to_assemble
            .entry(session)
            .or_insert_with(|| MessageToAssemble::new(total));

        let is_valid =
            Assembler::is_valid_index(fragment.fragment_index, total, to_assemble.pieces_number);
        if !is_valid {
            return Err(FragmentMalformed {});
        }

        Ok(to_assemble
            .pieces
            .insert(fragment.fragment_index, fragment)
            .is_none())
    }

    /// Check wheter the index are valid of not. To be valid the `expected_total`
    /// should be the same as the `total` in the new packet, and the `index` should be
    /// between 0 <= `index` < `total`.
    /// # Return
    /// True if valid and false else.
    pub(super) fn is_valid_index(
        index: FragmentIdx,
        total: FragmentIdx,
        expected_total: FragmentIdx,
    ) -> bool {
        index < total && total == expected_total
    }
}
