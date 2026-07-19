use crate::{FitId, FleetId, ItemId};

#[derive(Copy, Clone)]
pub enum FleetIdBackref {
    Id(FleetId),
    Backref(usize),
}

#[derive(Copy, Clone)]
pub enum FitIdBackref {
    Id(FitId),
    Backref(usize),
}

#[derive(Copy, Clone)]
pub enum ItemIdBackref {
    Id(ItemId),
    BackrefMain(usize),
    BackrefCharge(usize),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use super::*;

    const BACKREF_PREFIX: &str = "#";
    const CHARGE_SUFFIX: &str = "c";

    impl<'de> serde::Deserialize<'de> for FleetIdBackref {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = FleetIdBackref;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("fleet ID, or #-prefixed backreference")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if let Some(value_str) = v.strip_prefix(BACKREF_PREFIX) {
                        let index = usize::from_str(value_str).map_err(|e| serde::de::Error::custom(e))?;
                        return Ok(Self::Value::Backref(index));
                    }
                    let fleet_id = FleetId::from_str(v).map_err(|e| serde::de::Error::custom(e))?;
                    Ok(Self::Value::Id(fleet_id))
                }
            }
            deserializer.deserialize_str(Visitor)
        }
    }

    impl<'de> serde::Deserialize<'de> for FitIdBackref {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = FitIdBackref;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("fit ID, or #-prefixed backreference")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if let Some(value_str) = v.strip_prefix(BACKREF_PREFIX) {
                        let index = usize::from_str(value_str).map_err(|e| serde::de::Error::custom(e))?;
                        return Ok(Self::Value::Backref(index));
                    }
                    let fit_id = FitId::from_str(v).map_err(|e| serde::de::Error::custom(e))?;
                    Ok(Self::Value::Id(fit_id))
                }
            }
            deserializer.deserialize_str(Visitor)
        }
    }

    impl<'de> serde::Deserialize<'de> for ItemIdBackref {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = ItemIdBackref;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("item ID, or #-prefixed backreference")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if let Some(value_str) = v.strip_prefix(BACKREF_PREFIX) {
                        if let Some(value_str) = value_str.strip_suffix(CHARGE_SUFFIX) {
                            let index = usize::from_str(value_str).map_err(|e| serde::de::Error::custom(e))?;
                            return Ok(Self::Value::BackrefCharge(index));
                        }
                        let index = usize::from_str(value_str).map_err(|e| serde::de::Error::custom(e))?;
                        return Ok(Self::Value::BackrefMain(index));
                    }
                    let item_id = ItemId::from_str(v).map_err(|e| serde::de::Error::custom(e))?;
                    Ok(Self::Value::Id(item_id))
                }
            }
            deserializer.deserialize_str(Visitor)
        }
    }
}
