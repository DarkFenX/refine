use crate::util::round_f64_to_i32;

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct SlotIndex(i32);
impl SlotIndex {
    pub const fn from_i32(value: i32) -> Self {
        Self(value)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SlotIndex {
    pub(crate) fn from_f64_rounded(value: f64) -> Self {
        Self(round_f64_to_i32(value))
    }
}
impl From<SlotIndex> for i32 {
    fn from(v: SlotIndex) -> Self {
        v.0
    }
}
