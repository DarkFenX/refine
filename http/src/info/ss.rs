use crate::info::{FitInfo, FitInfoMode, ItemInfoMode, SsInfoMode};

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum SsInfo {
    IdOnly(String),
    Full(SsInfoFull),
}
impl SsInfo {
    pub(crate) fn mk_info(
        ss_id: String,
        core_ss: &mut reefast::SolarSystem,
        ss_mode: SsInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Self {
        match ss_mode {
            SsInfoMode::IdOnly => Self::IdOnly(ss_id),
            SsInfoMode::Full => Self::Full(SsInfoFull::mk_info(ss_id, core_ss, fit_mode, item_mode)),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct SsInfoFull {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fits: Vec<FitInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fleets: Vec<()>,
}
impl SsInfoFull {
    fn mk_info(
        ss_id: String,
        core_ss: &mut reefast::SolarSystem,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Self {
        Self {
            id: ss_id,
            fits: core_ss
                .get_fit_ids()
                .iter()
                .map(|v| FitInfo::mk_info(core_ss, v, fit_mode, item_mode))
                .collect(),
            fleets: Vec::new(),
        }
    }
}
