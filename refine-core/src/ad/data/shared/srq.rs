use crate::ad::AItemId;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AModifierSrq {
    SelfRef,
    ItemId(AItemId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde-ad")]
mod custom_serde {
    use serde::{
        de::{Deserialize, Deserializer, Error, Visitor},
        ser::{Serialize, Serializer},
    };

    use super::*;

    impl Serialize for AModifierSrq {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SelfRef => serializer.serialize_none(),
                Self::ItemId(id) => id.serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for AModifierSrq {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AModifierSrq;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("null or integer")
                }

                fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::SelfRef)
                }
                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::SelfRef)
                }

                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::ItemId(AItemId::from_i32(
                        v.try_into().map_err(Error::custom)?,
                    )))
                }
                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::ItemId(AItemId::from_i32(
                        v.try_into().map_err(Error::custom)?,
                    )))
                }
                fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::ItemId(AItemId::from_i32(v as i32)))
                }
            }

            deserializer.deserialize_any(VisitorImpl)
        }
    }
}
