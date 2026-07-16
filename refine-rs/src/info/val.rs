use crate::val::{FitValInfoDetails, SolValInfoDetails};

pub struct FitValInfo {
    pub passed: bool,
    pub details: Option<FitValInfoDetails>,
}

pub struct SolValInfo {
    pub passed: bool,
    pub details: Option<SolValInfoDetails>,
}
