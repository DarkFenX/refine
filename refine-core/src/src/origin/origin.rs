/// Was data source read from cache or generated from passed EVE data.
#[derive(Clone)]
pub enum SrcOrigin {
    /// Data was generated from scratch, with a reason why.
    Generated(SrcOriginGeneratedReason),
    /// Cached data was used, with fingerprint string.
    Cached(String),
}

/// Reason why data was generated from scratch.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(tag = "type", rename_all = "snake_case", content = "message")
)]
#[derive(Clone)]
pub enum SrcOriginGeneratedReason {
    NoCacher,
    /// EVE data handler did not return EVE data version, with error message which was returned.
    NoEveDataVersion(String),
    /// Adapted data cacher did not return cached data fingerprint, with error message which was
    /// returned.
    NoCachedFingerprint(String),
    /// Needed fingerprint and cached fingerprint were different, with message which mentions both.
    FingerprintMismatch(String),
    /// Adapted data cacher was unable to read cache data, with error message it returned.
    CacheLoadFailed(String),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Serialize, SerializeStruct, Serializer};

    use super::*;

    impl Serialize for SrcOrigin {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Generated(reason) => {
                    let mut variant = serializer.serialize_struct("Generated", 2)?;
                    variant.serialize_field("type", "generated")?;
                    variant.serialize_field("reason", reason)?;
                    variant.end()
                }
                Self::Cached(fingerprint) => {
                    let mut variant = serializer.serialize_struct("Cached", 2)?;
                    variant.serialize_field("type", "cached")?;
                    variant.serialize_field("fingerprint", fingerprint)?;
                    variant.end()
                }
            }
        }
    }
}
