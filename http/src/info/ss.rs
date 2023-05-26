use crate::info::{FitInfo, FitInfoMode, ItemInfoMode, SolSysInfoMode};

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum SolSysInfo {
    IdOnly(String),
    Full(SolSysInfoFull),
}
impl SolSysInfo {
    pub(crate) fn mk_info(
        ss_id: String,
        core_ss: &mut reefast::SolarSystem,
        ss_mode: SolSysInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Self {
        match ss_mode {
            SolSysInfoMode::IdOnly => Self::IdOnly(ss_id),
            SolSysInfoMode::Full => Self::Full(SolSysInfoFull::mk_info(ss_id, core_ss, fit_mode, item_mode)),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct SolSysInfoFull {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fits: Vec<FitInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fleets: Vec<()>,
}
impl SolSysInfoFull {
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
