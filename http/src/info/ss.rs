use crate::info::{HFitInfo, HFitInfoMode, HItemInfoMode, HSsInfoMode};

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HSsInfo {
    Id(String),
    Full(HSsInfoFull),
}
impl HSsInfo {
    pub(crate) fn mk_info(
        ss_id: String,
        core_ss: &mut rc::SolarSystem,
        ss_mode: HSsInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Self {
        match ss_mode {
            HSsInfoMode::Id => Self::Id(ss_id),
            HSsInfoMode::Full => Self::Full(HSsInfoFull::mk_info(ss_id, core_ss, fit_mode, item_mode)),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct HSsInfoFull {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fits: Vec<HFitInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fleets: Vec<()>,
}
impl HSsInfoFull {
    fn mk_info(ss_id: String, core_ss: &mut rc::SolarSystem, fit_mode: HFitInfoMode, item_mode: HItemInfoMode) -> Self {
        Self {
            id: ss_id,
            fits: core_ss
                .get_fit_ids()
                .iter()
                .map(|fit_id| HFitInfo::mk_info(core_ss, fit_id, fit_mode, item_mode))
                .collect(),
            fleets: Vec::new(),
        }
    }
}
