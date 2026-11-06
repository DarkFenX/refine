#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct ShortIndex(u32);
impl ShortIndex {
    pub(super) fn from_usize(index: usize) -> Self {
        // Assume count of items can never exceed 32 bit space
        Self(index as u32)
    }
    pub(super) fn into_usize(self) -> usize {
        self.0 as usize
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ShortIndex {
    pub(super) const NO_SLOT: Self = Self(u32::MAX);
}
