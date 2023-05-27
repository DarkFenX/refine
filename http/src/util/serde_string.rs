use serde::{de, Deserialize, Deserializer, Serializer};

// Code taken from https://github.com/serde-rs/json/issues/329
pub(crate) fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: Serializer,
{
    serializer.collect_str(value)
}

pub(crate) fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
}
