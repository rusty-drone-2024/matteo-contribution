use crate::backend::disassembler::{Disassembler, Split};
use common_structs::types::SessionId;
use wg_2024::network::NodeId;

impl Disassembler {
    #[must_use]
    pub fn require_flood(&mut self) -> bool {
        let required = self.new_waiting > 0;
        self.new_waiting = 0;
        required
    }

    pub fn add_session_to_wait_queue(&mut self, session: SessionId) -> Result<&mut Split, String> {
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

    pub fn ready_sessions_waiting_for(&mut self, dest: NodeId) {
        if let Some(waitings) = self.waiting.remove(&dest) {
            let entry = self.finished_waiting.entry(dest).or_default();
            entry.extend(waitings);
        }
    }

    pub fn take_ready_session(&mut self) -> Vec<SessionId> {
        let mut res = vec![];
        for (_, sessions) in self.finished_waiting.drain() {
            res.extend(sessions);
        }
        res
    }
}
