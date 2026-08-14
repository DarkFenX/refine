use crate::val::{FitValInfoDetails, SolValInfoDetails};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Info modes
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum ValInfoMode {
    Simple,
    Detailed,
}
const impl Default for ValInfoMode {
    fn default() -> Self {
        Self::Detailed
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct ValInfoModes {
    #[cfg_attr(feature = "serde", serde(default))]
    pub validation: ValInfoMode = ValInfoMode::default(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Infos
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct FitValInfo {
    pub passed: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "custom_serde::skip_details_fit"))]
    pub details: Option<FitValInfoDetails>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SolValInfo {
    pub passed: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "custom_serde::skip_details_sol"))]
    pub details: Option<SolValInfoDetails>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use super::*;

    pub(super) fn skip_details_fit(details: &Option<FitValInfoDetails>) -> bool {
        match details {
            Some(details) => details.all_passed(),
            None => true,
        }
    }

    pub(super) fn skip_details_sol(details: &Option<SolValInfoDetails>) -> bool {
        match details {
            Some(details) => details.all_passed(),
            None => true,
        }
    }
}
