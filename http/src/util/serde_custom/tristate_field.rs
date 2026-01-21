// Base decoding part is inspired by https://github.com/serde-rs/serde/issues/1042

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::de::{DeserializeAs, DeserializeAsWrap};

#[derive(Default)]
pub(crate) enum TriStateField<T> {
    Value(T),
    None,
    #[default]
    Absent,
}
impl<T> TriStateField<T> {
    pub(crate) fn is_absent(&self) -> bool {
        matches!(self, Self::Absent)
    }
}
impl<T> From<Option<T>> for TriStateField<T> {
    fn from(val: Option<T>) -> Self {
        match val {
            Some(inner) => Self::Value(inner),
            None => Self::None,
        }
    }
}
impl<T, E> From<Result<T, E>> for TriStateField<T> {
    fn from(val: Result<T, E>) -> Self {
        match val {
            Ok(inner) => Self::Value(inner),
            Err(_) => Self::None,
        }
    }
}

impl<T> Serialize for TriStateField<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            // This still serializes None, still have to declare "skip_serializing_if" in containing
            // struct
            TriStateField::Absent => serializer.serialize_unit(),
            TriStateField::None => serializer.serialize_none(),
            TriStateField::Value(value) => value.serialize(serializer),
        }
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
        formatter.write_str("TriStateField<T>")
    }

    #[inline]
    fn visit_none<E>(self) -> Result<TriStateField<T>, E>
    where
        E: serde::de::Error,
    {
        Ok(TriStateField::None)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(TriStateField::Value)
    }

    #[inline]
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
        Ok(
            match TriStateField::<DeserializeAsWrap<T, U>>::deserialize(deserializer)? {
                TriStateField::Value(v) => TriStateField::Value(v.into_inner()),
                TriStateField::None => TriStateField::None,
                TriStateField::Absent => TriStateField::Absent,
            },
        )
    }
}
