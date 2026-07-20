use crate::val::{FitValInfoDetails, SolValInfoDetails};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FitValInfo {
    pub passed: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub details: Option<FitValInfoDetails>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SolValInfo {
    pub passed: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub details: Option<SolValInfoDetails>,
}
