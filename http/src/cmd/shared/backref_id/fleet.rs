use std::str::FromStr;

use serde::Deserialize;

use super::shared::BACKREF_PREFIX;

pub(in crate::cmd) enum HFleetIdBackref {
    Id(rc::FleetId),
    Backref(usize),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization support
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'de> Deserialize<'de> for HFleetIdBackref {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HFleetIdBackrefVisitor;

        impl<'de> serde::de::Visitor<'de> for HFleetIdBackrefVisitor {
            type Value = HFleetIdBackref;

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
                let fleet_id = rc::FleetId::from_str(v).map_err(|e| serde::de::Error::custom(e))?;
                Ok(Self::Value::Id(fleet_id))
            }
        }
        deserializer.deserialize_str(HFleetIdBackrefVisitor)
    }
}
