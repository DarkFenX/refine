#[derive(Clone)]
pub enum StatResult<T, EO, EI> {
    NotRequested,
    Result(Vec<Result<T, EI>>),
    Error(EO),
}
impl<T, EO, EI> StatResult<T, EO, EI> {
    pub fn unwrap(self) -> Vec<Result<T, EI>>
    where
        EO: std::fmt::Debug,
    {
        match self {
            Self::NotRequested => panic!("called StatResult::unwrap() on a `NotRequested` value"),
            Self::Result(stat) => stat,
            Self::Error(e) => panic!("called StatResult::unwrap() on a `Error` value: {:?}", e),
        }
    }
    pub fn is_not_requested(&self) -> bool {
        matches!(self, StatResult::NotRequested)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// No errors whatsoever
impl<T> StatResult<T, !, !> {
    pub(crate) fn from_stat(stat: T) -> Self {
        Self::Result(vec![Ok(stat)])
    }
}
// All errors become outer errors
impl<T, E> StatResult<T, E, !> {
    pub(crate) fn from_result_outer(result: Result<T, E>) -> Self {
        match result {
            Ok(stat) => Self::Result(vec![Ok(stat)]),
            Err(err) => Self::Error(err),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Serialize, SerializeSeq, Serializer};

    use super::*;

    impl<T, EO, EI> Serialize for StatResult<T, EO, EI>
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
                Self::Result(stats) => {
                    let mut seq = serializer.serialize_seq(Some(stats.len()))?;
                    for stat in stats.iter() {
                        seq.serialize_element(&stat.as_ref().ok())?;
                    }
                    seq.end()
                }
                Self::Error(..) => serializer.serialize_none(),
            }
        }
    }
}
