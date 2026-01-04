use crate::util::f64_to_i32;

type InnerSlotIndex = i32;

#[derive(Copy, Clone, Default)]
pub struct SlotIndex(InnerSlotIndex);
impl SlotIndex {
    pub fn new(value: InnerSlotIndex) -> Self {
        Self(value)
    }
    pub(crate) fn new_f64(value: f64) -> Self {
        Self(f64_to_i32(value))
    }
}
impl From<i32> for SlotIndex {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}
