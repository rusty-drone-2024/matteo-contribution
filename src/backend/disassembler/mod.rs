mod fragment_info;
mod split;
mod test;
mod waiting;

pub use crate::backend::disassembler::split::Split;
use common_structs::message::Message;
use common_structs::types::{FragmentIdx, Session};
use std::collections::{HashMap, HashSet};
use wg_2024::network::NodeId;

#[derive(Default)]
pub struct Disassembler {
    splits: HashMap<Session, Split>,
    new_waiting: usize,
    waiting: HashMap<NodeId, HashSet<Session>>,
    finished_waiting: HashMap<NodeId, HashSet<Session>>,
}

impl Disassembler {
    pub fn split(&mut self, session: Session, dest: NodeId, msg: Message) {
        let fragments = msg.into_fragments();
        let split = Split::new(dest, fragments);
        self.splits.insert(session, split);
    }

    #[allow(dead_code)]
    pub fn get(&self, session: Session) -> Option<&Split> {
        self.splits.get(&session)
    }

    pub fn get_mut(&mut self, session: Session) -> Option<&mut Split> {
        self.splits.get_mut(&session)
    }

    pub fn ack(&mut self, session: Session, fragment_idx: FragmentIdx) -> Result<bool, String> {
        let split = self.splits.get_mut(&session).ok_or("No session id")?;

        let res = split.ack(fragment_idx)?;
        let is_acked = split.is_acked();

        if res && is_acked {
            self.splits.remove(&session);
        }
        Ok(res)
    }
}
