use crate::{
    ad::ACount,
    util::{round_f64_to_u32, trunc_f64_to_u32},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Count(u32);
impl Count {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
    pub fn into_inner(self) -> u32 {
        self.0
    }
    pub(crate) fn new_f64_trunced(value: f64) -> Self {
        Self(trunc_f64_to_u32(value))
    }
    pub(crate) fn new_f64_rounded(value: f64) -> Self {
        Self(round_f64_to_u32(value))
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
