use full::HFitInfoFull;
use id::HFitInfoId;

use crate::{
    info::{HFitInfoMode, HItemInfoMode},
    util::HResult,
};

mod full;
mod id;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HFitInfo {
    Id(HFitInfoId),
    Full(HFitInfoFull),
}
impl HFitInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HResult<Self> {
        let info = match fit_mode {
            HFitInfoMode::Id => Self::Id(HFitInfoId::mk_info(core_sol, fit_id)?),
            HFitInfoMode::Full => Self::Full(HFitInfoFull::mk_info(core_sol, fit_id, item_mode)?),
        };
        Ok(info)
    }
}
