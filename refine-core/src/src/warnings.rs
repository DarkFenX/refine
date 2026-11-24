use crate::ad::{AData, ADataWarnings};

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
        let Self {
            eve_data_fetch,
            adg_pk_duplicates,
            adg_cleanup,
            adg_validation,
            adg_conversion_main,
            adg_customization,
            adg_conversion_aux,
            cache_write,
        } = self;
        eve_data_fetch.is_empty()
            && adg_pk_duplicates.is_empty()
            && adg_cleanup.is_empty()
            && adg_validation.is_empty()
            && adg_conversion_main.is_empty()
            && adg_customization.is_empty()
            && adg_conversion_aux.is_empty()
            && cache_write.is_none()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcWarnings {
    pub(super) fn from_adapted_warnings(a_data: &mut AData) -> Self {
        let ADataWarnings {
            data_fetch,
            pk_duplicates,
            cleanup,
            validation,
            conversion_main,
            customization,
            conversion_aux,
        } = std::mem::take(&mut a_data.warnings);
        Self {
            eve_data_fetch: data_fetch,
            adg_pk_duplicates: pk_duplicates,
            adg_cleanup: cleanup,
            adg_validation: validation,
            adg_conversion_main: conversion_main,
            adg_customization: customization,
            adg_conversion_aux: conversion_aux,
            cache_write: None,
        }
    }
}
