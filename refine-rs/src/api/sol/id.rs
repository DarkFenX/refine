#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct SolarSystemId(uuid::Uuid);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystemId {
    pub(super) fn new() -> Self {
        SolarSystemId(uuid::Uuid::new_v4())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
pub use custom_serde::ParseSolarSystemIdError;

#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use super::*;

    impl FromStr for SolarSystemId {
        type Err = ParseSolarSystemIdError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let inner = uuid::Uuid::try_parse(s)?;
            Ok(Self(inner))
        }
    }

    #[derive(thiserror::Error, Debug)]
    #[error("{0}")]
    pub struct ParseSolarSystemIdError(#[from] uuid::Error);
}
