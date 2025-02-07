use crate::disassembler::fragment_info::FragmentInfo;
use common_structs::types::FragmentIdx;
use std::collections::{HashMap, HashSet};
use wg_2024::network::NodeId;
use wg_2024::packet::Fragment;

/// Represent a single `Message` split in fragments (`pieces`).
/// It also contains `destination`, acknoledge status and to send status.
pub struct Split {
    /// The destinations.
    destination: NodeId,
    /// The number of acked fragments.
    n_acked: usize,
    /// All the fragments with their acked status.
    pieces: HashMap<FragmentIdx, FragmentInfo>,
    /// Wheter all message are to be sent. It override the `to_send` field.
    /// Only one at the time can be true (`all_to_send` or not empty `to_send`)
    all_to_send: bool,
    /// The fragment yet to be sent.
    to_send: HashSet<FragmentIdx>,
}

impl Split {
    /// Create a new `Split` with the given `destination` and `fragments`
    /// On creationg all fragments are set to be sent and not acked.
    pub(super) fn new(destination: NodeId, fragments: Vec<Fragment>) -> Self {
        let fragments = fragments.into_iter().enumerate();
        let pieces = fragments.map(|(i, frag)| (i as FragmentIdx, frag.into()));

        Self {
            destination,
            n_acked: 0,
            pieces: pieces.collect(),
            to_send: HashSet::default(),
            all_to_send: true,
        }
    }

    /// Getter of destination
    /// # Return
    /// The destination of the message
    pub const fn destination(&self) -> NodeId {
        self.destination
    }

    /// Getter of ack status
    /// # Return
    /// Wheter the fragment is acked or not.
    pub(super) fn is_acked(&self) -> bool {
        self.n_acked >= self.pieces.len()
    }

    /// Ack a fragment, with given `fragment_idx`.
    /// # Return
    /// whether or not the ack was successful.
    /// # Errors
    /// In case the fragment is not in the message
    pub(super) fn ack(&mut self, fragment_idx: FragmentIdx) -> Result<bool, String> {
        let successful = self
            .pieces
            .get_mut(&fragment_idx)
            .ok_or("Couldn't find the fragment_idx")?
            .ack();
        if successful {
            self.n_acked += 1;
        }
        Ok(successful)
    }

    /// Set a fragment to be waited to send.
    /// # return
    /// true in case of success (false if already waiting for that fragment)
    pub fn wait_for(&mut self, fragment_idx: FragmentIdx) -> bool {
        if self.all_to_send {
            return false;
        }
        self.to_send.insert(fragment_idx)
    }

    /// Take all fragment that are to send
    /// All the fragment will be removed by the waiting list and
    /// it is the responsability of the caller to put back in waiting
    /// fragment that failed to send.
    /// # Return
    /// The vec of fragments to be sent.
    #[must_use]
    pub fn take_to_send(&mut self) -> Vec<Fragment> {
        if self.all_to_send {
            self.all_to_send = false;
            return self.fragments();
        }

        let waiting: Vec<FragmentIdx> = self.to_send.drain().collect();
        let mut res = Vec::with_capacity(waiting.len());

        for id in waiting {
            if let Some(piece) = self.pieces.get(&id) {
                res.push(piece.fragment().clone());
            }
        }
        res
    }

    /// Getter of all fragments (not in order)
    /// # Return
    /// All the fragments (waiting and not, acked and not)
    pub(super) fn fragments(&self) -> Vec<Fragment> {
        self.pieces
            .values()
            .map(FragmentInfo::fragment)
            .cloned()
            .collect()
    }
}
