#![allow(dead_code)]
use super::{Assembler, ToAssemble};

impl Assembler {
    pub(super) fn expect(&mut self, session_id: u64) {
        if !self.is_expecting(session_id) {
            self.expecting_session_ids
                .insert(session_id, ToAssemble::default());
        }
    }

    pub(super) fn stop_expecting(&mut self, session_id: u64) {
        self.expecting_session_ids.remove(&session_id);
    }

    pub(super) fn is_expecting(&self, session_id: u64) -> bool {
        self.expecting_session_ids.contains_key(&session_id)
    }
}
