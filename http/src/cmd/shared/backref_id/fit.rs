use std::str::FromStr;

use serde::Deserialize;

const BACKREF_PREFIX: &str = "#";

pub(in crate::cmd) enum HFitIdBackref {
    Id(rc::FitId),
    Backref(usize),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization support
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'de> Deserialize<'de> for HFitIdBackref {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HFitIdBackrefVisitor;

        impl<'de> serde::de::Visitor<'de> for HFitIdBackrefVisitor {
            type Value = HFitIdBackref;

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
                let fit_id = rc::FitId::from_str(v).map_err(|e| serde::de::Error::custom(e))?;
                Ok(Self::Value::Id(fit_id))
            }
        }
        deserializer.deserialize_str(HFitIdBackrefVisitor)
    }
}
