mod fragment_info;
mod split;
mod test;
mod waiting;

pub use crate::disassembler::split::Split;
use common_structs::message::Message;
use common_structs::types::{FragmentIdx, Session};
use std::collections::{HashMap, HashSet};
use wg_2024::network::NodeId;

/// Disassembler contains all message that the leaf is trying
/// to send. Both waiting to send and waiting to ack.
#[derive(Default)]
pub struct Disassembler {
    /// Messages splitted.
    splits: HashMap<Session, Split>,
    /// The number of new waiting since last checked.
    new_waiting: usize,
    /// The session of messages waiting for a path to a node.
    waiting: HashMap<NodeId, HashSet<Session>>,
    /// The session of messages that found a a path to a node.
    finished_waiting: HashMap<NodeId, HashSet<Session>>,
}

impl Disassembler {
    /// Split a `msg` in splits and store them with the given `session` and
    /// `destination`.
    /// If the session already existed it is replaced.
    // TODO That is probably not good
    pub fn split(&mut self, session: Session, dest: NodeId, msg: Message) {
        let fragments = msg.into_fragments();
        let split = Split::new(dest, fragments);
        self.splits.insert(session, split);
    }

    /// Getter for a single `Split` based on the `session`
    /// # Return
    /// A reference to the split if it is present.
    #[allow(dead_code)]
    pub fn get(&self, session: Session) -> Option<&Split> {
        self.splits.get(&session)
    }

    /// Getter for a single `Split` based on the `session`
    /// # Return
    /// A mutable reference to the split if it is present.
    pub fn get_mut(&mut self, session: Session) -> Option<&mut Split> {
        self.splits.get_mut(&session)
    }

    /// Mark a fragment decided by the `fragment_idx` of a message decided by the `session`
    /// # Return
    /// Wheater the `Message` is fully acker or not.
    /// # Error
    /// It return error if the `session` is not present in the disassembler.
    pub fn ack(&mut self, session: Session, fragment_idx: FragmentIdx) -> Result<bool, String> {
        let split = self.splits.get_mut(&session).ok_or("No session id")?;

        let res = split.ack(fragment_idx)?;
        let is_acked = split.is_acked();

        if res && is_acked {
            self.splits.remove(&session);
            return Ok(true);
        }
        Ok(false)
    }
}
