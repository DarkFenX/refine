#[derive(Copy, Clone, Default)]
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
    pub fn map<U, F>(self, f: F) -> TriStateField<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Value(x) => TriStateField::Value(f(x)),
            Self::None => TriStateField::None,
            Self::Absent => TriStateField::Absent,
        }
    }
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(..))
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::{
        de::{Deserialize, Deserializer, Error, Visitor},
        ser::{Serialize, Serializer},
    };

    use super::*;

    impl<T> Serialize for TriStateField<T>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                // This still serializes None, still have to declare "skip_serializing_if" in
                // containing struct
                Self::Absent => serializer.serialize_unit(),
                Self::None => serializer.serialize_none(),
                Self::Value(value) => value.serialize(serializer),
            }
        }
    }

    impl<'de, T> Deserialize<'de> for TriStateField<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl<T> {
                phantom: std::marker::PhantomData<T>,
            }

            impl<'de, T> Visitor<'de> for VisitorImpl<T>
            where
                T: Deserialize<'de>,
            {
                type Value = TriStateField<T>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("TriStateField<T>")
                }

                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::None)
                }
                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    T::deserialize(deserializer).map(Self::Value::Value)
                }
                fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::None)
                }
            }

            deserializer.deserialize_option(VisitorImpl {
                phantom: std::marker::PhantomData,
            })
        }
    }
}
