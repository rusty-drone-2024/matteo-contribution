mod fragment_info;
mod split;
mod test;
mod waiting;

pub use crate::backend::disassembler::split::Split;
use common_structs::message::Message;
use common_structs::types::{FragmentIndex, SessionId};
use std::collections::{HashMap, HashSet};
use wg_2024::network::NodeId;

#[derive(Default)]
pub struct Disassembler {
    splits: HashMap<SessionId, Split>,
    new_waiting: usize,
    waiting: HashMap<NodeId, HashSet<SessionId>>,
    finished_waiting: HashMap<NodeId, HashSet<SessionId>>,
}

impl Disassembler {
    pub fn split(&mut self, session: SessionId, dest: NodeId, msg: Message) {
        let fragments = msg.into_fragments();
        let split = Split::new(dest, fragments);

        if self.waiting.entry(dest).or_default().insert(session){
            self.new_waiting += 1;
        }
        self.splits.insert(session, split);
    }

    pub fn get(&self, session: SessionId) -> Option<&Split> {
        self.splits.get(&session)
    }

    pub fn get_mut(&mut self, session: SessionId) -> Option<&mut Split> {
        self.splits.get_mut(&session)
    }

    pub fn ack(&mut self, session: SessionId, fragment_id: FragmentIndex) -> Result<bool, String> {
        let split = self.splits.get_mut(&session).ok_or("No session id")?;

        let res = split.ack(fragment_id)?;
        let is_acked = split.is_acked();

        if res && is_acked {
            self.splits.remove(&session);
        }
        Ok(res)
    }
}
