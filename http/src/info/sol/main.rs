use serde::Serialize;

use super::{full::HSolInfoFull, id::HSolInfoId};
use crate::info::{HFitInfoMode, HFleetInfoMode, HItemInfoMode, HSolInfoMode};

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
            HSolInfoMode::Id => Self::Id(HSolInfoId::from_sol_id(sol_id)),
            HSolInfoMode::Full => Self::Full(HSolInfoFull::mk_info(sol_id, core_sol, fleet_mode, fit_mode, item_mode)),
        }
    }
}
