use rc::Lender;
use serde::Serialize;

use crate::{
    info::{HFitInfo, HFitInfoMode, HFleetInfo, HFleetInfoMode, HItemInfo, HItemInfoMode},
    shared::{HDpsProfile, HSecZone, HSpool},
};

#[derive(Serialize)]
pub(crate) struct HSolInfoFull {
    id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fleets: Vec<HFleetInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fits: Vec<HFitInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    sw_effects: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    proj_effects: Vec<HItemInfo>,
    sec_zone: HSecZone,
    default_spool: HSpool,
    default_incoming_dps: HDpsProfile,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolInfoFull {
    pub(in crate::info::sol) fn from_id_and_core(
        sol_id: String,
        core_sol: &mut rc::SolarSystem,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: sol_id,
            fleets: core_sol
                .iter_fleets_mut()
                .map_into_iter(|mut core_fleet| HFleetInfo::from_core(&mut core_fleet, fleet_mode))
                .collect(),
            fits: core_sol
                .iter_fits_mut()
                .map_into_iter(|mut core_fit| HFitInfo::from_core(&mut core_fit, fit_mode, item_mode))
                .collect(),
            sw_effects: core_sol
                .iter_sw_effects_mut()
                .map_into_iter(|mut core_sw_effect| HItemInfo::from_core_sw_effect(&mut core_sw_effect, item_mode))
                .collect(),
            proj_effects: core_sol
                .iter_proj_effects_mut()
                .map_into_iter(|mut proj_effect| HItemInfo::from_core_proj_effect(&mut proj_effect, item_mode))
                .collect(),
            sec_zone: HSecZone::from_core(core_sol.get_sec_zone()),
            default_spool: HSpool::from_core(core_sol.get_default_spool()),
            default_incoming_dps: HDpsProfile::from_core(core_sol.get_default_incoming_dps()),
        }
    }
}
