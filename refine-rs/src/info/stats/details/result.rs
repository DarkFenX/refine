pub enum StatResult<T, E> {
    NotRequested,
    Result(Vec<T>),
    Error(E),
}
impl<T, E> StatResult<T, E> {
    pub fn is_not_requested(&self) -> bool {
        matches!(self, StatResult::NotRequested)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T, E> From<Result<T, E>> for StatResult<T, E> {
    fn from(val: Result<T, E>) -> Self {
        match val {
            Ok(stat) => Self::Result(vec![stat]),
            Err(err) => Self::Error(err),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Serialize, Serializer};

    use super::*;

    impl<T, E> Serialize for StatResult<T, E>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                // This still serializes null for JSON, so have to declare "skip_serializing_if" in
                // parent struct
                Self::NotRequested => serializer.serialize_unit(),
                Self::Result(value) => value.serialize(serializer),
                Self::Error(_) => serializer.serialize_none(),
            }
        }
    }
}
