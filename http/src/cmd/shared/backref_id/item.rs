use std::str::FromStr;

use serde::Deserialize;

use super::shared::BACKREF_PREFIX;

const CHARGE_SUFFIX: &str = "c";

pub(in crate::cmd) enum HItemIdBackref {
    Id(rc::ItemId),
    BackrefMain(usize),
    BackrefCharge(usize),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization support
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'de> Deserialize<'de> for HItemIdBackref {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HItemIdBackrefVisitor;

        impl<'de> serde::de::Visitor<'de> for HItemIdBackrefVisitor {
            type Value = HItemIdBackref;

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
                let item_id = rc::ItemId::from_str(v).map_err(|e| serde::de::Error::custom(e))?;
                Ok(Self::Value::Id(item_id))
            }
        }
        deserializer.deserialize_str(HItemIdBackrefVisitor)
    }
}
