#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct CseqSlotPosition(u32);
impl CseqSlotPosition {
    pub(super) fn from_usize(position: usize) -> Self {
        // Slots in use (one per item and cycling options code built since the last clear) never
        // come close to u32::MAX, which is taken by NONE
        Self(position as u32)
    }
    pub(super) fn into_usize(self) -> usize {
        self.0 as usize
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CseqSlotPosition {
    pub(super) const NONE: Self = Self(u32::MAX);
}
