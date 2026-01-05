use crate::ed::EBuffId;

const EVE_PREFIX: &str = "e";
const CUSTOM_PREFIX: &str = "c";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ABuffId {
    Eve(AEveBuffId),
    Custom(ACustomBuffId),
}
impl std::fmt::Display for ABuffId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "{EVE_PREFIX}{id}"),
            Self::Custom(id) => write!(f, "{CUSTOM_PREFIX}{id}"),
        }
    }
}
impl std::str::FromStr for ABuffId {
    type Err = ABuffIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(id_str) = s.strip_prefix(EVE_PREFIX) {
            return Ok(Self::Eve(AEveBuffId::from_str(id_str)?));
        }
        if let Some(id_str) = s.strip_prefix(CUSTOM_PREFIX) {
            return Ok(Self::Custom(ACustomBuffId::from_str(id_str)?));
        }
        Err(ABuffIdParseError::InvalidPrefix)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ABuffIdParseError {
    #[error("invalid prefix, expected \"{eve}\" or \"{custom}\" prefix", eve = EVE_PREFIX, custom = CUSTOM_PREFIX)]
    InvalidPrefix,
    #[error("invalid int: {0}")]
    InvalidInt(#[from] std::num::ParseIntError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct AEveBuffId(i32);
impl AEveBuffId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct ACustomBuffId(i32);
impl ACustomBuffId {
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
impl const From<EBuffId> for ABuffId {
    fn from(buff_eid: EBuffId) -> Self {
        Self::Eve(AEveBuffId(buff_eid.into_inner()))
    }
}
