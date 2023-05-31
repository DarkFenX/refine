use crate::info::{FitInfoMode, ItemInfo, ItemInfoMode};

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum FitInfo {
    Id(String),
    Detailed(FitInfoDetailed),
}
impl FitInfo {
    pub(crate) fn mk_info(
        core_ss: &mut reefast_core::SolarSystem,
        fit_id: &reefast_core::ReeId,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Self {
        match fit_mode {
            FitInfoMode::IdOnly => Self::Id(fit_id.to_string()),
            FitInfoMode::Full => Self::Detailed(FitInfoDetailed::mk_info(core_ss, fit_id, item_mode)),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct ModuleRacks {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) high: Vec<ItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) mid: Vec<ItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) low: Vec<ItemInfo>,
}
impl ModuleRacks {
    fn from_infos(high: Vec<ItemInfo>, mid: Vec<ItemInfo>, low: Vec<ItemInfo>) -> Self {
        Self { high, mid, low }
    }
    fn is_empty(&self) -> bool {
        self.high.is_empty() && self.mid.is_empty() && self.low.is_empty()
    }
}

#[derive(serde::Serialize)]
pub(crate) struct FitInfoDetailed {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ship: Option<ItemInfo>,
    #[serde(skip_serializing_if = "ModuleRacks::is_empty")]
    pub(crate) modules: ModuleRacks,
}
impl FitInfoDetailed {
    fn mk_info(core_ss: &mut reefast_core::SolarSystem, fit_id: &reefast_core::ReeId, item_mode: ItemInfoMode) -> Self {
        let ship = core_ss
            .get_fit_ship_info(&fit_id)
            .ok()
            .map(|v| ItemInfo::mk_info(core_ss, &v, item_mode));
        let modules_high = core_ss
            .get_module_infos(&fit_id, reefast_core::ModRack::High)
            .iter()
            .map(|v| ItemInfo::mk_info(core_ss, v, item_mode))
            .collect();
        let modules_mid = core_ss
            .get_module_infos(&fit_id, reefast_core::ModRack::Mid)
            .iter()
            .map(|v| ItemInfo::mk_info(core_ss, v, item_mode))
            .collect();
        let modules_low = core_ss
            .get_module_infos(&fit_id, reefast_core::ModRack::Low)
            .iter()
            .map(|v| ItemInfo::mk_info(core_ss, v, item_mode))
            .collect();
        Self {
            id: fit_id.to_string(),
            ship,
            modules: ModuleRacks::from_infos(modules_high, modules_mid, modules_low),
        }
    }
}
