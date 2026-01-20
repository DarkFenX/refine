use serde::Serialize;

use super::{
    detailed::{HFitValResultDetailed, HSolValResultDetailed},
    simple::HValResultSimple,
};

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HSolValResult {
    Simple(HValResultSimple),
    Detailed(Box<HSolValResultDetailed>),
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HFitValResult {
    Simple(HValResultSimple),
    Detailed(Box<HFitValResultDetailed>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolValResult {
    pub(crate) fn from_core_simple(core_result: bool) -> Self {
        Self::Simple(HValResultSimple::from_core(core_result))
    }
    pub(crate) fn from_core_detailed(core_result: rc::val::ValResultSol) -> Self {
        Self::Detailed(Box::new(HSolValResultDetailed::from_core(core_result)))
    }
}

impl HFitValResult {
    pub(crate) fn from_core_simple(core_result: bool) -> Self {
        Self::Simple(HValResultSimple::from_core(core_result))
    }
    pub(crate) fn from_core_detailed(core_result: rc::val::ValResultFit) -> Self {
        Self::Detailed(Box::new(HFitValResultDetailed::from_core(core_result)))
    }
}
