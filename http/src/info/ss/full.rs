use crate::info::{HFitInfo, HFitInfoMode, HItemInfoMode};

#[derive(serde::Serialize)]
pub(crate) struct HSsInfoFull {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fits: Vec<HFitInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fleets: Vec<()>,
}
impl HSsInfoFull {
    pub(in crate::info::ss) fn mk_info(
        core_id: String,
        core_ss: &mut rc::SolarSystem,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: core_id,
            fits: core_ss
                .get_fit_ids()
                .iter()
                .map(|fit_id| HFitInfo::mk_info(core_ss, fit_id, fit_mode, item_mode))
                .collect(),
            fleets: Vec::new(),
        }
    }
}
