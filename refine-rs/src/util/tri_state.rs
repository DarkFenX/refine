#[derive(Default)]
pub enum TriStateField<T> {
    Value(T),
    None,
    #[default]
    Absent,
}
impl<T> TriStateField<T> {
    pub fn as_ref(&self) -> TriStateField<&T> {
        match *self {
            Self::Value(ref x) => TriStateField::Value(x),
            Self::None => TriStateField::None,
            Self::Absent => TriStateField::Absent,
        }
    }
    pub fn is_absent(&self) -> bool {
        matches!(self, Self::Absent)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
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

impl<T> From<Option<T>> for TriStateField<Vec<T>> {
    fn from(val: Option<T>) -> Self {
        match val {
            Some(inner) => Self::Value(vec![inner]),
            None => Self::None,
        }
    }
}
impl<T, E> From<Result<T, E>> for TriStateField<Vec<T>> {
    fn from(val: Result<T, E>) -> Self {
        match val {
            Ok(inner) => Self::Value(vec![inner]),
            Err(_) => Self::None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::Deserialize;

    use super::*;

    impl<T> serde::Serialize for TriStateField<T>
    where
        T: serde::Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                // This still serializes None, still have to declare "skip_serializing_if" in
                // containing struct
                TriStateField::Absent => serializer.serialize_unit(),
                TriStateField::None => serializer.serialize_none(),
                TriStateField::Value(value) => value.serialize(serializer),
            }
        }
    }

    impl<'de, T> Deserialize<'de> for TriStateField<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct Visitor<T> {
                marker: std::marker::PhantomData<T>,
            }

            impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
            where
                T: Deserialize<'de>,
            {
                type Value = TriStateField<T>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("TriStateField<T>")
                }

                #[inline]
                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::None)
                }

                #[inline]
                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    T::deserialize(deserializer).map(Self::Value::Value)
                }

                #[inline]
                fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::None)
                }
            }

            deserializer.deserialize_option(Visitor::<T> {
                marker: std::marker::PhantomData,
            })
        }
    }

    impl<'de, T, U> serde_with::DeserializeAs<'de, TriStateField<T>> for TriStateField<U>
    where
        U: serde_with::DeserializeAs<'de, T>,
    {
        fn deserialize_as<D>(deserializer: D) -> Result<TriStateField<T>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Ok(
                match TriStateField::<serde_with::de::DeserializeAsWrap<T, U>>::deserialize(deserializer)? {
                    TriStateField::Value(v) => TriStateField::Value(v.into_inner()),
                    TriStateField::None => TriStateField::None,
                    TriStateField::Absent => TriStateField::Absent,
                },
            )
        }
    }
}
