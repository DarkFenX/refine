use crate::ad::AData;

/// Info about data source: how it was made, and what warnings were encountered while making it.
#[derive(Clone)]
pub struct SrcInfo {
    pub origin: SrcOrigin,
    pub warnings: SrcWarnings,
}

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

/// Non-fatal problems encountered while building a data source.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SrcWarnings {
    /// Warnings recorded by EVE data handler.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub eve_data_fetch: Vec<String>,
    /// Info on removed data due to primary key collisions.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_pk_duplicates: Vec<String>,
    /// Cleanup stats.
    ///
    /// Those are purely informative, real data is expected to have lots of cleaned entries.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_cleanup: Vec<String>,
    /// Warnings encountered during data validation.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_validation: Vec<String>,
    /// Warnings encountered during data conversion.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_conversion_main: Vec<String>,
    /// Warnings encountered during data customization.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_customization: Vec<String>,
    /// Warnings encountered during post-customization data conversion.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_conversion_aux: Vec<String>,
    /// Errors recorded by adapted data cacher during writing.
    ///
    /// Those errors are not fatal for functioning of the lib, so might consider those as warnings.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cache_write: Option<String>,
}
impl SrcWarnings {
    pub fn is_empty(&self) -> bool {
        self.eve_data_fetch.is_empty()
            && self.adg_pk_duplicates.is_empty()
            && self.adg_cleanup.is_empty()
            && self.adg_validation.is_empty()
            && self.adg_conversion_main.is_empty()
            && self.adg_customization.is_empty()
            && self.adg_conversion_aux.is_empty()
            && self.cache_write.is_none()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcWarnings {
    pub(super) fn from_adapted_warnings(a_data: &mut AData) -> Self {
        Self {
            eve_data_fetch: std::mem::take(&mut a_data.warnings.data_fetch),
            adg_pk_duplicates: std::mem::take(&mut a_data.warnings.pk_duplicates),
            adg_cleanup: std::mem::take(&mut a_data.warnings.cleanup),
            adg_validation: std::mem::take(&mut a_data.warnings.validation),
            adg_conversion_main: std::mem::take(&mut a_data.warnings.conversion_main),
            adg_customization: std::mem::take(&mut a_data.warnings.customization),
            adg_conversion_aux: std::mem::take(&mut a_data.warnings.conversion_aux),
            cache_write: None,
        }
    }
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
