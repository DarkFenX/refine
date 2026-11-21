use crate::{
    FitId,
    val::{ValNotLoadedItemFail, ValResult, ValResultFit},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Validation result for a solar system.
///
/// Contains results of all the requested fits, and result for items not belonging to any fit.
///
/// Serialization notes: fits which have no failed validations are not serialized.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ValResultSol {
    #[cfg_attr(
        feature = "serde",
        serde(
            skip_serializing_if = "custom_serde::skip_fits",
            serialize_with = "custom_serde::serialize_fits"
        )
    )]
    pub fits: Vec<(FitId, ValResultFit)>,
    /// Not loaded stand-alone items - i.e. items which do not belong to any fits.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub not_loaded_item: ValResult<ValNotLoadedItemFail>,
}
impl ValResultSol {
    pub fn all_passed(&self) -> bool {
        let Self { fits, not_loaded_item } = self;
        fits.iter().all(|(_, fit_result)| fit_result.all_passed()) && !not_loaded_item.is_failed()
    }
    pub fn get_fit(&self, fit_id: &FitId) -> Option<&ValResultFit> {
        self.fits.iter().find_map(|(id, fit_result)| match id == fit_id {
            true => Some(fit_result),
            false => None,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ValResultSol {
    pub(in crate::svc::vast) fn with_fit_capacity(capacity: usize) -> Self {
        Self {
            fits: Vec::with_capacity(capacity),
            not_loaded_item: ValResult::NotRequested,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::Serializer;

    use super::*;

    pub(super) fn skip_fits(fits: &[(FitId, ValResultFit)]) -> bool {
        fits.iter().all(|(_, fit_result)| fit_result.all_passed())
    }

    pub(super) fn serialize_fits<S>(fits: &[(FitId, ValResultFit)], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let failed_fits = fits
            .iter()
            .filter(|(_, fit_result)| !fit_result.all_passed())
            .map(|(fit_id, fit_result)| (fit_id, fit_result));
        serializer.collect_map(failed_fits)
    }
}
