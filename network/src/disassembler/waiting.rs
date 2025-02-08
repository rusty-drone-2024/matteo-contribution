use crate::disassembler::{Disassembler, Split};
use common_structs::types::Session;
use wg_2024::network::NodeId;

impl Disassembler {
    /// Take the information about wheter a flood is required.
    /// If this called twice in a row it will always return false the second
    /// time as taking this information, reset the requirement of a flood to false.
    /// # Return
    /// Wheter or not a flood is required
    #[must_use]
    pub fn require_flood(&mut self, aggressive: bool) -> bool {
        let required = self.new_waiting > 0;
        self.new_waiting = 0;
        required || (aggressive && !self.waiting.is_empty())
    }

    /// Add a message (decided by the `session` in the waiting queue for a specific node.
    /// This does not change which fragments are in the wait queue. It is assumed
    /// that some fragment are in the wait queue or will be put in the wait queue.
    /// # Return
    /// The split relative to the message in question which can be used to add
    /// fragments in the wait queue.
    /// # Errors
    /// In case the message relative to the `session` is not present.
    pub fn add_session_to_wait_queue(&mut self, session: Session) -> Result<&mut Split, String> {
        let Some(split) = self.splits.get_mut(&session) else {
            return Err("Split is missing".to_string());
        };

        let dest = split.destination();

        let entry = self.waiting.entry(dest).or_default();
        if entry.insert(session) {
            self.new_waiting += 1;
        }
        Ok(split)
    }

    /// Mark all message that are waiting for a particular node `dest` as ready.
    /// As ready they will be able to be taken with the method `take_ready_session`
    pub fn ready_sessions_waiting_for(&mut self, dest: NodeId) {
        if let Some(waitings) = self.waiting.remove(&dest) {
            let entry = self.finished_waiting.entry(dest).or_default();
            entry.extend(waitings);
        }
    }

    /// Take all ready session. As taking remove them from the ready queue it is
    /// the responsability of the caller to send the message or to stash it again in the wait queue.
    /// # Return
    /// A vector of all the session that are ready.
    #[must_use]
    pub fn take_ready_session(&mut self) -> Vec<Session> {
        let mut res = vec![];
        for (_, sessions) in self.finished_waiting.drain() {
            res.extend(sessions);
        }
        res
    }
}
