use crate::disassembler::fragment_info::FragmentInfo;
use common_structs::types::FragmentIdx;
use std::collections::HashSet;
use wg_2024::network::NodeId;
use wg_2024::packet::Fragment;

pub struct Split {
    destination: NodeId,
    n_acked: usize,
    pieces: Vec<FragmentInfo>,
    all_to_send: bool,
    to_send: HashSet<FragmentIdx>,
}

impl Split {
    pub(super) fn new(destination: NodeId, fragments: Vec<Fragment>) -> Self {
        let pieces = fragments.into_iter().map(FragmentInfo::new);

        Self {
            destination,
            n_acked: 0,
            pieces: pieces.collect(),
            to_send: HashSet::default(),
            all_to_send: true,
        }
    }

    pub fn destination(&self) -> NodeId {
        self.destination
    }

    pub(super) fn is_acked(&self) -> bool {
        self.n_acked >= self.pieces.len()
    }

    /// # Return
    /// whether or not the ack was successful
    pub(super) fn ack(&mut self, fragment_idx: FragmentIdx) -> Result<bool, String> {
        let successful = self.get_mut_piece(fragment_idx)?.ack();
        if successful {
            self.n_acked += 1;
        }
        Ok(successful)
    }

    /// # return
    /// true in case of success (false if already waiting)
    pub fn wait_for(&mut self, fragment_idx: FragmentIdx) -> bool {
        if self.all_to_send {
            return false;
        }
        self.to_send.insert(fragment_idx)
    }

    pub fn take_to_send(&mut self) -> Vec<Fragment> {
        if self.all_to_send {
            self.all_to_send = false;
            return self.fragments();
        }

        let waiting: Vec<FragmentIdx> = self.to_send.drain().collect();
        let mut res = Vec::with_capacity(waiting.len());

        for id in waiting {
            if let Ok(piece) = self.get_piece(id) {
                res.push(piece.fragment().clone());
            }
        }
        res
    }

    pub(super) fn fragments(&self) -> Vec<Fragment> {
        self.pieces
            .iter()
            .map(FragmentInfo::fragment)
            .cloned()
            .collect()
    }

    fn get_piece(&self, fragment_idx: FragmentIdx) -> Result<&FragmentInfo, String> {
        let index = usize::try_from(fragment_idx)
            .map_err(|_| "Exiting range of Vec (problem of protocol)".to_string())?;
        self.pieces
            .get(index)
            .ok_or("Couldn't find fragment of message".to_string())
    }

    fn get_mut_piece(&mut self, fragment_idx: FragmentIdx) -> Result<&mut FragmentInfo, String> {
        let index = usize::try_from(fragment_idx)
            .map_err(|_| "Exiting range of Vec (problem of protocol)")?;
        self.pieces
            .get_mut(index)
            .ok_or("Couldn't find fragment of message".to_string())
    }
}
