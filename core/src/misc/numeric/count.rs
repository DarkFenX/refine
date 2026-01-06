use crate::{
    ad::ACount,
    util::{round_f64_to_u32, trunc_f64_to_u32},
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, derive_more::Display)]
pub struct Count(u32);
impl Count {
    pub const fn from_u32(value: u32) -> Self {
        Self(value)
    }
    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Count {
    pub(crate) const ZERO: Self = Self(0);
    pub(crate) const ONE: Self = Self(1);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Count {
    pub(crate) fn from_f64_trunced(value: f64) -> Self {
        Self(trunc_f64_to_u32(value))
    }
    pub(crate) fn from_f64_rounded(value: f64) -> Self {
        Self(round_f64_to_u32(value))
    }
    pub(crate) fn from_a_count(a_count: ACount) -> Self {
        Self(a_count.into_u32())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
// Addition
impl std::ops::AddAssign<Count> for Count {
    fn add_assign(&mut self, rhs: Count) {
        self.0 += rhs.0;
    }
}
// Subtraction
impl std::ops::Sub<u32> for Count {
    type Output = Count;
    fn sub(self, rhs: u32) -> Self::Output {
        Count::from_u32(self.0 - rhs)
    }
}
