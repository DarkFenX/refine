use std::num::Wrapping;

use crate::util::{LibDefault, LibIncrement};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct FleetId(u32);
impl LibDefault for FleetId {
    fn lib_default() -> Self {
        Self(0)
    }
}
impl LibIncrement for FleetId {
    fn lib_increment(&mut self) {
        self.0 = (Wrapping(self.0) + Wrapping(1)).0;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Error
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(thiserror::Error, Debug)]
#[error("fleet {fleet_id} not found")]
pub struct FleetFoundError {
    pub fleet_id: FleetId,
}
// Conversion needed for unified user entity container to work
impl From<FleetId> for FleetFoundError {
    fn from(fleet_id: FleetId) -> Self {
        Self { fleet_id }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
pub use custom_serde::ParseFleetIdError;

#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use super::*;

    impl FromStr for FleetId {
        type Err = ParseFleetIdError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let raw = u32::from_str(s)?;
            Ok(Self(raw))
        }
    }

    #[derive(thiserror::Error, Debug)]
    #[error("{0}")]
    pub struct ParseFleetIdError(#[from] std::num::ParseIntError);

    impl serde::Serialize for FleetId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            let string = format!("{self}");
            serializer.serialize_str(&string)
        }
    }

    impl<'de> serde::Deserialize<'de> for FleetId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = FleetId;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("string with integer")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Self::Value::from_str(v).map_err(serde::de::Error::custom)
                }
            }

            deserializer.deserialize_str(Visitor)
        }
    }
}
