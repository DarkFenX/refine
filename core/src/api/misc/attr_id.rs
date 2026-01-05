use crate::ad::{AAttrId, ACustomAttrId, AEveAttrId};

const EVE_PREFIX: &str = "e";
const CUSTOM_PREFIX: &str = "c";

/// ID of an attribute.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AttrId {
    /// ID of an EVE attribute.
    Eve(EveAttrId),
    /// ID of an attribute created by the library.
    Custom(CustomAttrId),
}
impl std::fmt::Display for AttrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "{EVE_PREFIX}{id}"),
            Self::Custom(id) => write!(f, "{CUSTOM_PREFIX}{id}"),
        }
    }
}
impl std::str::FromStr for AttrId {
    type Err = AttrIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(id_str) = s.strip_prefix(EVE_PREFIX) {
            return Ok(Self::Eve(EveAttrId::from_str(id_str)?));
        }
        if let Some(id_str) = s.strip_prefix(CUSTOM_PREFIX) {
            return Ok(Self::Custom(CustomAttrId::from_str(id_str)?));
        }
        Err(AttrIdParseError::InvalidPrefix)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum AttrIdParseError {
    #[error("invalid prefix, expected \"{eve}\" or \"{custom}\" prefix", eve = EVE_PREFIX, custom = CUSTOM_PREFIX)]
    InvalidPrefix,
    #[error("invalid int: {0}")]
    InvalidInt(#[from] std::num::ParseIntError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct EveAttrId(i32);
impl EveAttrId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct CustomAttrId(i32);
impl CustomAttrId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<AAttrId> for AttrId {
    fn from(attr_aid: AAttrId) -> Self {
        match attr_aid {
            AAttrId::Eve(id) => Self::Eve(EveAttrId::new(id.into_inner())),
            AAttrId::Custom(id) => Self::Custom(CustomAttrId::new(id.into_inner())),
        }
    }
}
impl From<&AAttrId> for AttrId {
    fn from(attr_aid: &AAttrId) -> Self {
        match attr_aid {
            AAttrId::Eve(id) => Self::Eve(EveAttrId::new(id.into_inner())),
            AAttrId::Custom(id) => Self::Custom(CustomAttrId::new(id.into_inner())),
        }
    }
}
impl From<AttrId> for AAttrId {
    fn from(attr_id: AttrId) -> Self {
        match attr_id {
            AttrId::Eve(id) => Self::Eve(AEveAttrId::new(id.into_inner())),
            AttrId::Custom(id) => Self::Custom(ACustomAttrId::new(id.into_inner())),
        }
    }
}
impl From<&AttrId> for AAttrId {
    fn from(attr_id: &AttrId) -> Self {
        match attr_id {
            AttrId::Eve(id) => Self::Eve(AEveAttrId::new(id.into_inner())),
            AttrId::Custom(id) => Self::Custom(ACustomAttrId::new(id.into_inner())),
        }
    }
}
