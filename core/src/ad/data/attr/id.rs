use crate::{ed::EAttrId, util::round_f64_to_i32};

const EVE_PREFIX: &str = "e";
const CUSTOM_PREFIX: &str = "c";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AAttrId {
    Eve(AEveAttrId),
    Custom(ACustomAttrId),
}
impl std::fmt::Display for AAttrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "{EVE_PREFIX}{id}"),
            Self::Custom(id) => write!(f, "{CUSTOM_PREFIX}{id}"),
        }
    }
}
impl std::str::FromStr for AAttrId {
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
    #[error("invalid int: {0}")]
    InvalidInt(#[from] std::num::ParseIntError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AEveAttrId(i32);
impl AEveAttrId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
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
}
impl AEveAttrId {
    pub(crate) fn from_f64_rounded(id: f64) -> Self {
        Self(round_f64_to_i32(id))
    }
}
