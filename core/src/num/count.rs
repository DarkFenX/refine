use crate::{
    ad::ACount,
    num::{PValue, Value},
    util::{ceil_f64_to_u32, round_f64_to_u32, trunc_f64_to_u32},
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
    pub(crate) const TWO: Self = Self(2);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Count {
    pub(crate) fn from_usize(value: usize) -> Self {
        Self(value.clamp(u32::MIN as usize, u32::MAX as usize) as u32)
    }
    pub(crate) fn into_usize(self) -> usize {
        self.0 as usize
    }
    pub(crate) fn from_f64_trunced(value: f64) -> Self {
        Self(trunc_f64_to_u32(value))
    }
    pub(crate) fn from_f64_rounded(value: f64) -> Self {
        Self(round_f64_to_u32(value))
    }
    pub(crate) fn from_value_trunced(value: Value) -> Self {
        Self(trunc_f64_to_u32(value.into_f64()))
    }
    pub(crate) fn from_value_rounded(value: Value) -> Self {
        Self(round_f64_to_u32(value.into_f64()))
    }
    pub(crate) fn from_value_ceiled(value: Value) -> Self {
        Self(ceil_f64_to_u32(value.into_f64()))
    }
    pub(crate) fn from_pvalue_trunced(value: PValue) -> Self {
        Self(trunc_f64_to_u32(value.into_f64()))
    }
    pub(crate) fn from_pvalue_rounded(value: PValue) -> Self {
        Self(round_f64_to_u32(value.into_f64()))
    }
    pub(crate) fn from_pvalue_ceiled(value: PValue) -> Self {
        Self(ceil_f64_to_u32(value.into_f64()))
    }
    pub(crate) fn from_a_count(a_count: ACount) -> Self {
        Self(a_count.into_u32())
    }
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64(self.0 as f64)
    }
    pub(crate) fn into_pvalue(self) -> PValue {
        PValue::from_f64_unchecked(self.0 as f64)
    }
}
impl From<Count> for u32 {
    fn from(v: Count) -> Self {
        v.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Iteration
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::iter::Step for Count {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        u32::steps_between(&start.0, &end.0)
    }
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        u32::forward_checked(start.0, count).map(Self)
    }
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        u32::backward_checked(start.0, count).map(Self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
// Addition
impl std::ops::Add<Count> for Count {
    type Output = Count;
    fn add(self, rhs: Count) -> Self::Output {
        Count(self.0 + rhs.0)
    }
}
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
impl std::ops::Sub<Count> for Count {
    type Output = Count;
    fn sub(self, rhs: Count) -> Self::Output {
        Count::from_u32(self.0 - rhs.0)
    }
}
impl std::ops::SubAssign<Count> for Count {
    fn sub_assign(&mut self, rhs: Count) {
        self.0 -= rhs.0;
    }
}
// Multiplication
impl std::ops::Mul<Count> for Count {
    type Output = Count;
    fn mul(self, rhs: Count) -> Self::Output {
        Count(self.0 * rhs.0)
    }
}
// Division
impl std::ops::Div<Count> for Count {
    type Output = Count;
    fn div(self, rhs: Count) -> Self::Output {
        Count(self.0 / rhs.0)
    }
}
// Remainder
impl std::ops::Rem<Count> for Count {
    type Output = Count;
    fn rem(self, rhs: Count) -> Self::Output {
        Count(self.0 % rhs.0)
    }
}
