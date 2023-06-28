use serde::{Deserialize, Deserializer};

// Modified example from https://github.com/serde-rs/serde/issues/1344
pub(in crate::phb) fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(i32::deserialize(deserializer)? != 0)
}
