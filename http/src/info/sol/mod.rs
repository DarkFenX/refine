use full::HSolInfoFull;
use id::HSolInfoId;
use serde::Serialize;

use crate::info::{HFitInfoMode, HFleetInfoMode, HItemInfoMode, HSolInfoMode};

mod full;
mod id;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HSolInfo {
    Id(HSolInfoId),
    Full(HSolInfoFull),
}
impl HSolInfo {
    pub(crate) fn mk_info(
        sol_id: String,
        core_sol: &mut rc::SolarSystem,
        sol_mode: HSolInfoMode,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Self {
        match sol_mode {
            HSolInfoMode::Id => Self::Id(sol_id.into()),
            HSolInfoMode::Full => Self::Full(HSolInfoFull::mk_info(sol_id, core_sol, fleet_mode, fit_mode, item_mode)),
        }
    }
}
