// Taken from https://github.com/serde-rs/serde/issues/1042

use serde::{Deserialize, Deserializer};

pub(crate) enum OptionalField<T> {
    Value(T),
    None,
    Absent,
}
impl<T> Default for OptionalField<T> {
    fn default() -> Self {
        OptionalField::Absent
    }
}

pub(crate) struct OptionalFieldVisitor<T> {
    marker: std::marker::PhantomData<T>,
}
impl<'de, T> Deserialize<'de> for OptionalField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(OptionalFieldVisitor::<T> {
            marker: std::marker::PhantomData,
        })
    }
}

impl<'de, T> serde::de::Visitor<'de> for OptionalFieldVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = OptionalField<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("OptionalField<T>")
    }

    fn visit_unit<E>(self) -> Result<OptionalField<T>, E>
    where
        E: serde::de::Error,
    {
        Ok(OptionalField::None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(OptionalField::Value)
    }
}
