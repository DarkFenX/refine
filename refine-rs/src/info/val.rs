use crate::val::FitValInfoDetails;

pub struct FitValInfo {
    pub passed: bool,
    pub details: Option<FitValInfoDetails>,
}
