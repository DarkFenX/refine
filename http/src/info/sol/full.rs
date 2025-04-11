use crate::{
    info::{HFitInfo, HFitInfoMode, HFleetInfo, HFleetInfoMode, HItemInfo, HItemInfoMode, MkItemInfo},
    shared::{HDpsProfile, HSecZone},
};

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
    pub(crate) sec_zone: HSecZone,
    pub(crate) default_incoming_dps: HDpsProfile,
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
                .get_fleet_infos()
                .iter()
                .filter_map(|core_fleet| HFleetInfo::mk_info(core_sol, &core_fleet.id, fleet_mode).ok())
                .collect(),
            fits: core_sol
                .get_fit_infos()
                .iter()
                .filter_map(|core_fit| HFitInfo::mk_info(core_sol, &core_fit.id, fit_mode, item_mode).ok())
                .collect(),
            sw_effects: core_sol
                .get_sw_effects()
                .iter()
                .map(|v| HItemInfo::mk_info(core_sol, v, item_mode))
                .collect(),
            proj_effects: core_sol
                .get_proj_effect_infos()
                .iter()
                .map(|v| HItemInfo::mk_info(core_sol, v, item_mode))
                .collect(),
            sec_zone: core_sol.get_sec_zone().into(),
            default_incoming_dps: (&core_sol.get_default_incoming_dps()).into(),
        }
    }
}
