use crate::info::{HItemInfo, HItemInfoMode, MkItemInfo};

#[derive(serde::Serialize)]
pub(crate) struct HFitInfoFull {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) character: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) skills: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) implants: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) boosters: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ship: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stance: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) subsystems: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "HModuleRacks::is_empty")]
    pub(crate) modules: HModuleRacks,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) rigs: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) drones: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fighters: Vec<HItemInfo>,
}
impl HFitInfoFull {
    pub(in crate::info::fit) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        fit_id: &rc::ReeId,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: fit_id.to_string(),
            character: core_ss
                .get_fit_character_info(&fit_id)
                .ok()
                .map(|v| HItemInfo::mk_info(core_ss, &v, item_mode)),
            skills: core_ss
                .get_fit_skill_infos(&fit_id)
                .unwrap()
                .iter()
                .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                .collect(),
            implants: core_ss
                .get_fit_implant_infos(&fit_id)
                .unwrap()
                .iter()
                .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                .collect(),
            boosters: core_ss
                .get_fit_booster_infos(&fit_id)
                .unwrap()
                .iter()
                .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                .collect(),
            ship: core_ss
                .get_fit_ship_info(&fit_id)
                .ok()
                .map(|v| HItemInfo::mk_info(core_ss, &v, item_mode)),
            stance: core_ss
                .get_fit_stance_info(&fit_id)
                .ok()
                .map(|v| HItemInfo::mk_info(core_ss, &v, item_mode)),
            subsystems: core_ss
                .get_fit_subsystem_infos(&fit_id)
                .unwrap()
                .iter()
                .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                .collect(),
            modules: HModuleRacks {
                high: core_ss
                    .get_module_infos(&fit_id, rc::ModRack::High)
                    .unwrap()
                    .iter()
                    .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                    .collect(),
                mid: core_ss
                    .get_module_infos(&fit_id, rc::ModRack::Mid)
                    .unwrap()
                    .iter()
                    .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                    .collect(),
                low: core_ss
                    .get_module_infos(&fit_id, rc::ModRack::Low)
                    .unwrap()
                    .iter()
                    .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                    .collect(),
            },
            rigs: core_ss
                .get_fit_rig_infos(&fit_id)
                .unwrap()
                .iter()
                .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                .collect(),
            drones: core_ss
                .get_fit_drone_infos(&fit_id)
                .unwrap()
                .iter()
                .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                .collect(),
            fighters: core_ss
                .get_fit_fighter_infos(&fit_id)
                .unwrap()
                .iter()
                .map(|v| HItemInfo::mk_info(core_ss, v, item_mode))
                .collect(),
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
    fn is_empty(&self) -> bool {
        self.high.is_empty() && self.mid.is_empty() && self.low.is_empty()
    }
}
