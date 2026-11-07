use std::str::FromStr;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public - general
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct SolId(uuid::Uuid);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public - custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FromStr for SolId {
    type Err = SolIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = uuid::Uuid::try_parse(s)?;
        Ok(Self(inner))
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct SolIdParseError(#[from] uuid::Error);

#[cfg(feature = "serde")]
mod custom_serde {
    use serde::{
        de::{Deserialize, Deserializer, Error, Visitor},
        ser::{Serialize, Serializer},
    };

    use super::*;

    impl Serialize for SolId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let string = format!("{self}");
            serializer.serialize_str(&string)
        }
    }

    impl<'de> Deserialize<'de> for SolId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = SolId;

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

            deserializer.deserialize_string(VisitorImpl)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolId {
    pub(crate) fn new() -> Self {
        SolId(uuid::Uuid::new_v4())
    }
}
