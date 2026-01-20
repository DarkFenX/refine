use serde::Serialize;

use super::{full::HFitInfoFull, id::HFitInfoId};
use crate::info::{HFitInfoMode, HItemInfoMode};

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HFitInfo {
    Id(HFitInfoId),
    Full(Box<HFitInfoFull>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitInfo {
    pub(crate) fn from_core(core_fit: &mut rc::FitMut, fit_mode: HFitInfoMode, item_mode: HItemInfoMode) -> Self {
        match fit_mode {
            HFitInfoMode::Id => Self::Id(HFitInfoId::from_core(core_fit)),
            HFitInfoMode::Full => Self::Full(Box::new(HFitInfoFull::from_core(core_fit, item_mode))),
        }
    }
}
