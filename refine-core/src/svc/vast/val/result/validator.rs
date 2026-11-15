////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Result of a single validation.
///
/// Serialization notes: serialized format is a sequence, which has status, followed by failure
/// details for statuses which have them.
#[derive(Clone)]
pub enum ValResult<T> {
    NotRequested,
    Passed,
    /// Validation is failed only by items which were passed as known failures.
    PassedWithKfs(T),
    Failed(T),
}
impl<T> ValResult<T> {
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(_))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> ValResult<T> {
    pub(in crate::svc::vast::val) fn from_fail(all_kfs: bool, fail: T) -> Self {
        match all_kfs {
            true => Self::PassedWithKfs(fail),
            false => Self::Failed(fail),
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

    impl<T> ValResult<T> {
        pub(in crate::svc::vast::val) fn is_not_requested(&self) -> bool {
            matches!(self, Self::NotRequested)
        }
    }

    impl<T> Serialize for ValResult<T>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let (status, fail) = match self {
                Self::NotRequested => ("not_requested", None),
                Self::Passed => ("passed", None),
                Self::PassedWithKfs(fail) => ("passed_with_kfs", Some(fail)),
                Self::Failed(fail) => ("failed", Some(fail)),
            };
            let len = match fail {
                Some(_) => 2,
                None => 1,
            };
            let mut seq = serializer.serialize_seq(Some(len))?;
            seq.serialize_element(status)?;
            if let Some(fail) = fail {
                seq.serialize_element(fail)?;
            }
            seq.end()
        }
    }
}
