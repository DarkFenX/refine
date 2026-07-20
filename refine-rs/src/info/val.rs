use crate::val::{FitValInfoDetails, SolValInfoDetails};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FitValInfo {
    pub passed: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_details_fit"))]
    pub details: Option<FitValInfoDetails>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SolValInfo {
    pub passed: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "skip_details_sol"))]
    pub details: Option<SolValInfoDetails>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
fn skip_details_fit(details: &Option<FitValInfoDetails>) -> bool {
    match details {
        Some(details) => details.all_passed(),
        None => true,
    }
}

#[cfg(feature = "serde")]
fn skip_details_sol(details: &Option<SolValInfoDetails>) -> bool {
    match details {
        Some(details) => details.all_passed(),
        None => true,
    }
}
