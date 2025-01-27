use crate::backend::disassembler::Disassembler;
use common_structs::types::{FragmentIndex, SessionId};
use wg_2024::network::NodeId;

impl Disassembler {
    #[must_use]
    pub fn require_flood(&mut self) -> bool {
        let required = self.new_waiting > 0;
        self.new_waiting = 0;
        required
    }

    pub fn add_waiting(&mut self, session: SessionId, dest: NodeId, fragment_id: FragmentIndex) {
        let Some(split) = self.splits.get_mut(&session) else {
            return;
        };

        split.add_waiting(fragment_id);
        self.waiting.entry(dest).or_default().push(session);
        self.new_waiting += 1;
    }

    pub fn remove_waiting_for(&mut self, destination: NodeId) {
        if let Some(waiting) = self.waiting.remove(&destination) {
            self.finished_waiting.insert(destination, waiting);
        }
    }

    pub fn take_ready(&mut self) -> Vec<SessionId> {
        let mut res = vec![];
        for (_, sessions) in self.finished_waiting.drain() {
            res.extend(sessions);
        }
        res
    }
}
