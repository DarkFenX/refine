use crate::info::{HFitInfo, HFitInfoMode, HFleetInfo, HFleetInfoMode, HItemInfo, HItemInfoMode, MkItemInfo};

#[derive(serde::Serialize)]
pub(crate) struct HSsInfoFull {
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
impl HSsInfoFull {
    pub(in crate::info::ss) fn mk_info(
        ss_id: String,
        core_ss: &mut rc::SolarSystem,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: ss_id,
            fleets: core_ss
                .get_fleet_ids()
                .iter()
                .filter_map(|fleet_id| HFleetInfo::mk_info(core_ss, fleet_id, fleet_mode).ok())
                .collect(),
            fits: core_ss
                .get_fit_ids()
                .iter()
                .filter_map(|fit_id| HFitInfo::mk_info(core_ss, fit_id, fit_mode, item_mode).ok())
                .collect(),
            sw_effects: core_ss
                .get_sw_effect_infos()
                .iter()
                .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                .collect(),
            proj_effects: core_ss
                .get_proj_effect_infos()
                .iter()
                .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                .collect(),
        }
    }
}
