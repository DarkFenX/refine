use crate::info::{HFitInfoMode, HItemInfo, HItemInfoMode};

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HFitInfo {
    Id(String),
    Detailed(HFitInfoDetailed),
}
impl HFitInfo {
    pub(crate) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        fit_id: &rc::ReeId,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Self {
        match fit_mode {
            HFitInfoMode::Id => Self::Id(fit_id.to_string()),
            HFitInfoMode::Full => Self::Detailed(HFitInfoDetailed::mk_info(core_ss, fit_id, item_mode)),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct HModuleRacks {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) high: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) mid: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) low: Vec<HItemInfo>,
}
impl HModuleRacks {
    fn from_infos(high: Vec<HItemInfo>, mid: Vec<HItemInfo>, low: Vec<HItemInfo>) -> Self {
        Self { high, mid, low }
    }
    fn is_empty(&self) -> bool {
        self.high.is_empty() && self.mid.is_empty() && self.low.is_empty()
    }
}

#[derive(serde::Serialize)]
pub(crate) struct HFitInfoDetailed {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ship: Option<HItemInfo>,
    #[serde(skip_serializing_if = "ModuleRacks::is_empty")]
    pub(crate) modules: HModuleRacks,
}
impl HFitInfoDetailed {
    fn mk_info(core_ss: &mut rc::SolarSystem, fit_id: &rc::ReeId, item_mode: HItemInfoMode) -> Self {
        let ship = core_ss
            .get_fit_ship_info(&fit_id)
            .ok()
            .map(|v| HItemInfo::mk_info(core_ss, &v, item_mode));
        let modules_high = core_ss
            .get_module_infos(&fit_id, rc::ModRack::High)
            .iter()
            .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
            .collect();
        let modules_mid = core_ss
            .get_module_infos(&fit_id, rc::ModRack::Mid)
            .iter()
            .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
            .collect();
        let modules_low = core_ss
            .get_module_infos(&fit_id, rc::ModRack::Low)
            .iter()
            .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
            .collect();
        Self {
            id: fit_id.to_string(),
            ship,
            modules: HModuleRacks::from_infos(modules_high, modules_mid, modules_low),
        }
    }
}
