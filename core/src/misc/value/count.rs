use crate::{ad::ACount, util::f64_to_u32};

#[derive(Copy, Clone, Default)]
pub struct Count(u32);
impl Count {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
    pub(crate) fn new_f64(value: f64) -> Self {
        Self(f64_to_u32(value))
    }
}
impl From<u32> for Count {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}
impl From<Count> for u32 {
    fn from(value: Count) -> Self {
        value.0
    }
}
// Conversions between lib-specific types
impl From<ACount> for Count {
    fn from(value: ACount) -> Self {
        Self::new(value.into_inner())
    }
}
