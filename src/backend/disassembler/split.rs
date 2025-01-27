use crate::backend::disassembler::fragment_info::FragmentInfo;
use common_structs::types::FragmentIndex;
use std::collections::HashSet;
use wg_2024::network::NodeId;
use wg_2024::packet::Fragment;

pub struct Split {
    destination: NodeId,
    n_acked: usize,
    pieces: Vec<FragmentInfo>,
    waiting: HashSet<FragmentIndex>,
    waiting_all: bool,
}

impl Split {
    pub(super) fn new(destination: NodeId, fragments: Vec<Fragment>) -> Self {
        let pieces = fragments.into_iter().map(FragmentInfo::new);

        Self {
            destination,
            n_acked: 0,
            pieces: pieces.collect(),
            waiting: HashSet::default(),
            waiting_all: true,
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
    pub(super) fn ack(&mut self, fragment_id: FragmentIndex) -> Result<bool, String> {
        let successful = self.get_mut_piece(fragment_id)?.ack();
        if successful {
            self.n_acked += 1;
        }
        Ok(successful)
    }

    pub fn get_fragment(&self, fragment_id: FragmentIndex) -> Result<Fragment, String> {
        Ok(self.get_piece(fragment_id)?.fragment().clone())
    }

    pub fn add_waiting(&mut self, fragment_id: FragmentIndex) -> bool {
        if self.waiting_all {
            return false;
        }
        self.waiting.insert(fragment_id)
    }

    pub fn add_all_to_waiting(&mut self) {
        self.waiting.clear();
        self.waiting_all = true;
    }

    pub fn take_waiting(&mut self) -> Vec<Fragment> {
        if self.waiting_all {
            self.waiting_all = false;
            return self.fragments();
        }

        let waiting: Vec<FragmentIndex> = self.waiting.drain().collect();
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

    fn get_piece(&self, fragment_id: FragmentIndex) -> Result<&FragmentInfo, String> {
        let index = usize::try_from(fragment_id)
            .map_err(|_| "Exiting range of Vec (problem of protocol)".to_string())?;
        self.pieces
            .get(index)
            .ok_or("Coundn't find fragment of message".to_string())
    }

    fn get_mut_piece(&mut self, fragment_id: FragmentIndex) -> Result<&mut FragmentInfo, String> {
        let index = usize::try_from(fragment_id)
            .map_err(|_| "Exiting range of Vec (problem of protocol)")?;
        self.pieces
            .get_mut(index)
            .ok_or("Coundn't find fragment of message".to_string())
    }
}
