use std::mem::replace;
use wg_2024::packet::Fragment;

/// Include in a single struct the info about the fragment.
/// which are the fragment itself and wheter it was acked.
pub struct FragmentInfo {
    fragment: Fragment,
    acked: bool,
}

impl From<Fragment> for FragmentInfo {
    /// Create a new `FragmentInfo` from the `fragment`. This
    /// is considered yet to ack.
    fn from(value: Fragment) -> Self {
        Self {
            fragment: value,
            acked: false,
        }
    }
}

impl FragmentInfo {
    /// Ack the fragments
    /// # Return
    /// whether or not the ack was successful
    pub(super) fn ack(&mut self) -> bool {
        !replace(&mut self.acked, true)
    }

    /// Getter method for fragment
    /// # Return
    /// The fragment in question
    pub(super) fn fragment(&self) -> &Fragment {
        &self.fragment
    }
}
