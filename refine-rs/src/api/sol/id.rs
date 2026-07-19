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

    use serde::{
        de::{Deserialize, Deserializer, Error, Visitor},
        ser::{Serialize, Serializer},
    };

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

    impl Serialize for SolarSystemId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let string = format!("{self}");
            serializer.serialize_str(&string)
        }
    }

    impl<'de> Deserialize<'de> for SolarSystemId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorState;

            impl<'de> Visitor<'de> for VisitorState {
                type Value = SolarSystemId;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("string with UUID")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Self::Value::from_str(v).map_err(Error::custom)
                }
            }

            deserializer.deserialize_str(VisitorState)
        }
    }
}
