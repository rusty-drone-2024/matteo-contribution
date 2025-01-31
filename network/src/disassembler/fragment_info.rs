use std::mem::replace;
use wg_2024::packet::Fragment;

pub struct FragmentInfo {
    fragment: Fragment,
    acked: bool,
}

impl FragmentInfo {
    pub(super) fn new(fragment: Fragment) -> Self {
        Self {
            fragment,
            acked: false,
        }
    }

    /// # Return
    /// whether or not the ack was successful
    pub(super) fn ack(&mut self) -> bool {
        !replace(&mut self.acked, true)
    }

    pub fn fragment(&self) -> &Fragment {
        &self.fragment
    }
}
