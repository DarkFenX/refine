/// Item's cycling options encoded as an item-type-specific code.
#[derive(Copy, Clone)]
pub(in crate::svc::cycle::item_getters) struct ItemCyclingOptionsCode(u8);
impl ItemCyclingOptionsCode {
    pub(super) const fn from_u8(code: u8) -> Self {
        Self(code)
    }
    pub(super) fn into_usize(self) -> usize {
        self.0 as usize
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemCyclingOptionsCode {
    // Count of distinct values codes take
    pub(super) const COUNT: usize = 4;
}
