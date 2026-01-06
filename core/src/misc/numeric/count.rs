use crate::{
    ad::ACount,
    util::{round_f64_to_u32, trunc_f64_to_u32},
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, derive_more::Display)]
pub struct Count(u32);
impl Count {
    pub(crate) const ZERO: Self = Self(0);
    pub(crate) const ONE: Self = Self(1);

    pub fn new(value: u32) -> Self {
        Self(value)
    }
    pub fn into_inner(self) -> u32 {
        self.0
    }
    pub(crate) fn from_f64_trunced(value: f64) -> Self {
        Self(trunc_f64_to_u32(value))
    }
    pub(crate) fn from_f64_rounded(value: f64) -> Self {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions between lib-specific types
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<ACount> for Count {
    fn from(value: ACount) -> Self {
        Self::new(value.into_u32())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
// Subtraction
impl std::ops::Sub<u32> for Count {
    type Output = Count;
    fn sub(self, rhs: u32) -> Self::Output {
        Count::new(self.0 - rhs)
    }
}
