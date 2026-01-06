use crate::{ed::EItemListId, util::round_f64_to_i32};

const EVE_PREFIX: &str = "e";
const CUSTOM_PREFIX: &str = "c";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AItemListId {
    Eve(AEveItemListId),
    Custom(ACustomItemListId),
}
impl std::fmt::Display for AItemListId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "{EVE_PREFIX}{id}"),
            Self::Custom(id) => write!(f, "{CUSTOM_PREFIX}{id}"),
        }
    }
}
impl std::str::FromStr for AItemListId {
    type Err = AItemListIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(id_str) = s.strip_prefix(EVE_PREFIX) {
            return Ok(Self::Eve(AEveItemListId::from_str(id_str)?));
        }
        if let Some(id_str) = s.strip_prefix(CUSTOM_PREFIX) {
            return Ok(Self::Custom(ACustomItemListId::from_str(id_str)?));
        }
        Err(AItemListIdParseError::InvalidPrefix)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum AItemListIdParseError {
    #[error("invalid prefix, expected \"{eve}\" or \"{custom}\" prefix", eve = EVE_PREFIX, custom = CUSTOM_PREFIX)]
    InvalidPrefix,
    #[error("invalid int: {0}")]
    InvalidInt(#[from] std::num::ParseIntError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AEveItemListId(i32);
impl AEveItemListId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct ACustomItemListId(i32);
impl ACustomItemListId {
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
impl AItemListId {
    pub(in crate::ad) const fn from_eid(item_list_eid: EItemListId) -> Self {
        Self::Eve(AEveItemListId(item_list_eid.into_i32()))
    }
}
impl AEveItemListId {
    pub(crate) fn from_f64_rounded(id: f64) -> Self {
        Self(round_f64_to_i32(id))
    }
}
