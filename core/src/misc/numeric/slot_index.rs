use crate::util::round_f64_to_i32;

#[derive(Copy, Clone)]
pub struct SlotIndex(i32);
impl SlotIndex {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
    pub(crate) fn new_f64_rounded(value: f64) -> Self {
        Self(round_f64_to_i32(value))
    }
}
impl From<i32> for SlotIndex {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}
