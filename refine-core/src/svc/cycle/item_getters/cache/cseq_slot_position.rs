#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct CseqSlotPosition(u32);
impl CseqSlotPosition {
    pub(super) fn from_usize(position: usize) -> Self {
        // Slots in use (one per item and cycling options code built since the last clear) never
        // come close to u32::MAX, where the constants are
        Self(position as u32)
    }
    pub(super) fn into_usize(self) -> usize {
        self.0 as usize
    }
    pub(super) fn is_slot(self) -> bool {
        // One comparison, since the constants take the top values
        self.0 < Self::NOT_CYCLING.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CseqSlotPosition {
    // Not built yet
    pub(super) const NONE: Self = Self(u32::MAX);
    // Built, but the item doesn't cycle, so it has no slot
    pub(super) const NOT_CYCLING: Self = Self(u32::MAX - 1);
}
