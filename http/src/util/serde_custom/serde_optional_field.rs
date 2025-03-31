// Taken from https://github.com/serde-rs/serde/issues/1042

use serde::{Deserialize, Deserializer};
use serde_with::de::{DeserializeAs, DeserializeAsWrap};

pub(crate) enum TriStateField<T> {
    Value(T),
    None,
    Absent,
}
impl<T> Default for TriStateField<T> {
    fn default() -> Self {
        TriStateField::Absent
    }
}

pub(crate) struct TriStateFieldVisitor<T> {
    marker: std::marker::PhantomData<T>,
}
impl<'de, T> Deserialize<'de> for TriStateField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(TriStateFieldVisitor::<T> {
            marker: std::marker::PhantomData,
        })
    }
}
impl<'de, T> serde::de::Visitor<'de> for TriStateFieldVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = TriStateField<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("OptionalField<T>")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(TriStateField::Value)
    }

    fn visit_unit<E>(self) -> Result<TriStateField<T>, E>
    where
        E: serde::de::Error,
    {
        Ok(TriStateField::None)
    }
}

impl<'de, T, U> DeserializeAs<'de, TriStateField<T>> for TriStateField<U>
where
    U: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<TriStateField<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(TriStateField::Value(
            DeserializeAsWrap::<T, U>::deserialize(deserializer)?.into_inner(),
        ))
    }
}
