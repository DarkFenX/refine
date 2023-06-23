use crate::info::{item::MkItemInfo, HItemInfo, HItemInfoMode};

#[derive(serde::Serialize)]
pub(crate) struct HFitInfoFull {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) character: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) implants: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ship: Option<HItemInfo>,
    #[serde(skip_serializing_if = "HModuleRacks::is_empty")]
    pub(crate) modules: HModuleRacks,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) rigs: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) drones: Vec<HItemInfo>,
}
impl HFitInfoFull {
    pub(in crate::info::fit) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        fit_id: &rc::ReeId,
        item_mode: HItemInfoMode,
    ) -> Self {
        let character = core_ss
            .get_fit_character_info(&fit_id)
            .ok()
            .map(|v| HItemInfo::mk_info(core_ss, &v, item_mode));
        let implants = core_ss
            .get_fit_implant_infos(&fit_id)
            .iter()
            .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
            .collect();
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
        let rigs = core_ss
            .get_fit_rig_infos(&fit_id)
            .iter()
            .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
            .collect();
        let drones = core_ss
            .get_fit_drone_infos(&fit_id)
            .iter()
            .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
            .collect();
        Self {
            id: fit_id.to_string(),
            character,
            implants,
            ship,
            modules: HModuleRacks::from_infos(modules_high, modules_mid, modules_low),
            rigs,
            drones,
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
