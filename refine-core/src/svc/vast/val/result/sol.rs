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
#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct ValResultSol {
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub fits: Vec<(FitId, ValResultFit)>,
    /// Not loaded stand-alone items - i.e. items which do not belong to any fits.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_requested"))]
    pub not_loaded_item: ValResult<ValNotLoadedItemFail>,
}
impl ValResultSol {
    pub fn all_passed(&self) -> bool {
        let Self { fits, not_loaded_item } = self;
        fits.iter().all(|(_, fit_result)| fit_result.all_passed()) && !not_loaded_item.is_failed()
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
