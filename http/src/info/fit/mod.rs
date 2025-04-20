use full::HFitInfoFull;
use id::HFitInfoId;

use crate::info::{HFitInfoMode, HItemInfoMode};

mod full;
mod id;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HFitInfo {
    Id(HFitInfoId),
    Full(Box<HFitInfoFull>),
}
impl HFitInfo {
    pub(crate) fn mk_info(core_fit: &mut rc::FitMut, fit_mode: HFitInfoMode, item_mode: HItemInfoMode) -> Self {
        match fit_mode {
            HFitInfoMode::Id => Self::Id(core_fit.into()),
            HFitInfoMode::Full => Self::Full(Box::new(HFitInfoFull::mk_info(core_fit, item_mode))),
        }
    }
}
