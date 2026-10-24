/// Zero-based position of an item within an ordered collection, e.g. of a module within its rack.
///
/// Serialization notes: deserialization clamps out-of-range input instead of failing.
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, derive_more::Display)]
pub struct Index(usize);
impl Index {
    pub const fn from_usize(index: usize) -> Self {
        Self(index)
    }
    pub const fn from_u64_clamped(index: u64) -> Self {
        Self(index.saturating_cast())
    }
    pub const fn from_i64_clamped(index: i64) -> Self {
        Self(index.saturating_cast())
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
impl Index {
    pub(crate) fn from_f64_rounded(index: f64) -> Self {
        Self(index.round() as usize)
    }
}
impl From<Index> for usize {
    fn from(v: Index) -> Self {
        v.0
    }
}
impl From<Index> for u32 {
    fn from(v: Index) -> Self {
        v.0.saturating_cast()
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use serde::de::{Deserialize, Deserializer, Error, Visitor};

    use super::*;

    impl FromStr for Index {
        type Err = IndexParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Ok(value) = i64::from_str(s) {
                return Ok(Self::from_i64_clamped(value));
            }
            let value = f64::from_str(s)?;
            if !value.is_finite() {
                return Err(IndexParseError::NotFinite(value));
            }
            let value = Self::from_f64_rounded(value);
            Ok(value)
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum IndexParseError {
        #[error(transparent)]
        InvalidNumber(#[from] std::num::ParseFloatError),
        #[error("value {0} is not finite")]
        NotFinite(f64),
    }

    struct VisitorImpl;
    impl Visitor<'_> for VisitorImpl {
        type Value = Index;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("number with index")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Index::from_u64_clamped(v))
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(Index::from_i64_clamped(v))
        }
        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match v.is_finite() {
                true => Ok(Index::from_f64_rounded(v)),
                false => Err(Error::custom(IndexParseError::NotFinite(v))),
            }
        }
    }

    impl<'de> Deserialize<'de> for Index {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_u64(VisitorImpl)
        }
    }
}
