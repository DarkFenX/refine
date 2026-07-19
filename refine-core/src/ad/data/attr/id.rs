use crate::{ed::EAttrId, util::round_f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AAttrId {
    Eve(AEveAttrId),
    Custom(ACustomAttrId),
}

#[cfg_attr(feature = "serde-ad", derive(derive_more::FromStr))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct AEveAttrId(i32);
impl AEveAttrId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

#[cfg_attr(feature = "serde-ad", derive(derive_more::FromStr))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct ACustomAttrId(i32);
impl ACustomAttrId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AAttrId {
    pub(in crate::ad) const fn from_eid(attr_eid: EAttrId) -> Self {
        Self::Eve(AEveAttrId(attr_eid.into_i32()))
    }
    pub(crate) fn try_eve_from_f64_rounded(id: f64) -> Option<Self> {
        Some(Self::Eve(AEveAttrId::try_from_f64_rounded(id)?))
    }
    pub(in crate::ad) fn dc_eve(&self) -> Option<EAttrId> {
        match self {
            Self::Eve(eve_attr_aid) => Some(EAttrId::from_i32(eve_attr_aid.into_i32())),
            Self::Custom(_) => None,
        }
    }
}
impl AEveAttrId {
    fn try_from_f64_rounded(id: f64) -> Option<Self> {
        match round_f64_to_i32(id) {
            // Reference to 0 is considered as no reference throughout EVE data
            0 => None,
            id => Some(Self(id)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
const EVE_PREFIX: &str = "e";
const CUSTOM_PREFIX: &str = "c";

impl std::fmt::Display for AAttrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "{EVE_PREFIX}{id}"),
            Self::Custom(id) => write!(f, "{CUSTOM_PREFIX}{id}"),
        }
    }
}

#[cfg(feature = "serde-ad")]
mod custom_serde_ad {
    use std::str::FromStr;

    use super::*;

    impl FromStr for AAttrId {
        type Err = AAttrIdParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Some(id_str) = s.strip_prefix(EVE_PREFIX) {
                return Ok(Self::Eve(AEveAttrId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(CUSTOM_PREFIX) {
                return Ok(Self::Custom(ACustomAttrId::from_str(id_str)?));
            }
            Err(AAttrIdParseError::InvalidPrefix)
        }
    }

    #[derive(thiserror::Error, Debug)]
    pub enum AAttrIdParseError {
        #[error("invalid prefix, expected \"{eve}\" or \"{custom}\" prefix", eve = EVE_PREFIX, custom = CUSTOM_PREFIX)]
        InvalidPrefix,
        #[error("{0}")]
        InvalidInt(#[from] std::num::ParseIntError),
    }
}
