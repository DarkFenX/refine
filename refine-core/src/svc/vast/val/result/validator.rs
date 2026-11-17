////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Result of a single validation.
///
/// Serialization notes: a failed validation is serialized as its failure details, any other result
/// is serialized as null. Results of a fit or of a solar system serialize only failed validations.
#[derive(Clone)]
pub enum ValResult<T> {
    NotRequested,
    Passed,
    Failed(T),
}
impl<T> ValResult<T> {
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(..))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Serialize, Serializer};

    use super::*;

    impl<T> ValResult<T> {
        pub(in crate::svc::vast::val) fn is_not_failed(&self) -> bool {
            !self.is_failed()
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
            match self {
                Self::Failed(fail) => serializer.serialize_some(fail),
                Self::NotRequested | Self::Passed => serializer.serialize_none(),
            }
        }
    }
}
