#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, derive_more::Display)]
pub struct Index(usize);
impl Index {
    pub const fn from_usize(index: usize) -> Self {
        Self(index)
    }
    pub const fn into_usize(self) -> usize {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Index {
    pub(crate) const ZERO: Self = Self(0);
    pub(crate) const ONE: Self = Self(1);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<Index> for usize {
    fn from(v: Index) -> Self {
        v.0
    }
}
impl From<Index> for u32 {
    fn from(v: Index) -> Self {
        v.0.min(u32::MAX as usize) as u32
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mathematics
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::ops::Add<Index> for Index {
    type Output = Index;
    fn add(self, rhs: Index) -> Self::Output {
        Index(self.0 + rhs.0)
    }
}
impl std::ops::AddAssign<Index> for Index {
    fn add_assign(&mut self, rhs: Index) {
        self.0 += rhs.0;
    }
}
impl std::ops::Sub<Index> for Index {
    type Output = Index;
    fn sub(self, rhs: Index) -> Self::Output {
        Index(self.0 - rhs.0)
    }
}
