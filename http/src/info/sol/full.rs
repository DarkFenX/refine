use crate::info::{HFitInfo, HFitInfoMode, HFleetInfo, HFleetInfoMode, HItemInfo, HItemInfoMode, MkItemInfo};

#[derive(serde::Serialize)]
pub(crate) struct HSolInfoFull {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fleets: Vec<HFleetInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fits: Vec<HFitInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) sw_effects: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) proj_effects: Vec<HItemInfo>,
}
impl HSolInfoFull {
    pub(in crate::info::sol) fn mk_info(
        sol_id: String,
        core_sol: &mut rc::SolarSystem,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: sol_id,
            fleets: core_sol
                .get_fleets()
                .iter()
                .filter_map(|core_fleet| HFleetInfo::mk_info(core_sol, &core_fleet.id, fleet_mode).ok())
                .collect(),
            fits: core_sol
                .get_fits()
                .iter()
                .filter_map(|core_fit| HFitInfo::mk_info(core_sol, &core_fit.id, fit_mode, item_mode).ok())
                .collect(),
            sw_effects: core_sol
                .get_sw_effect_infos()
                .iter()
                .map(|v| HItemInfo::mk_info(core_sol, v, item_mode))
                .collect(),
            proj_effects: core_sol
                .get_proj_effect_infos()
                .iter()
                .map(|v| HItemInfo::mk_info(core_sol, v, item_mode))
                .collect(),
        }
    }
}
