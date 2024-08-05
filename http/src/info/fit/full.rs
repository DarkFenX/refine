use itertools::Itertools;

use crate::{
    info::{HItemInfo, HItemInfoMode, MkItemInfo},
    util::HExecResult,
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitInfoFull {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolFitId,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fleet: Option<rc::SolFleetId>,
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fw_effects: Vec<HItemInfo>,
}
impl HFitInfoFull {
    pub(in crate::info::fit) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::SolFitId,
        item_mode: HItemInfoMode,
    ) -> HExecResult<Self> {
        let core_fit = core_sol.get_fit(fit_id)?;
        let fit = Self {
            id: *fit_id,
            fleet: core_fit.fleet,
            character: core_fit
                .character
                .map(|v| core_sol.get_item_info(&v).ok())
                .flatten()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode)),
            skills: core_fit
                .skills
                .iter()
                .filter_map(|v| core_sol.get_item_info(&v).ok())
                .collect_vec()
                .into_iter()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                .collect(),
            implants: core_fit
                .implants
                .iter()
                .filter_map(|v| core_sol.get_item_info(&v).ok())
                .collect_vec()
                .into_iter()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                .collect(),
            boosters: core_fit
                .boosters
                .iter()
                .filter_map(|v| core_sol.get_item_info(&v).ok())
                .collect_vec()
                .into_iter()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                .collect(),
            ship: core_fit
                .ship
                .map(|v| core_sol.get_item_info(&v).ok())
                .flatten()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode)),
            stance: core_fit
                .stance
                .map(|v| core_sol.get_item_info(&v).ok())
                .flatten()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode)),
            subsystems: core_fit
                .subsystems
                .iter()
                .filter_map(|v| core_sol.get_item_info(&v).ok())
                .collect_vec()
                .into_iter()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                .collect(),
            modules: HModuleRacks {
                high: core_fit
                    .mods_high
                    .iter()
                    .filter_map(|v| core_sol.get_item_info(&v).ok())
                    .collect_vec()
                    .into_iter()
                    .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                    .collect(),
                mid: core_fit
                    .mods_mid
                    .iter()
                    .filter_map(|v| core_sol.get_item_info(&v).ok())
                    .collect_vec()
                    .into_iter()
                    .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                    .collect(),
                low: core_fit
                    .mods_low
                    .iter()
                    .filter_map(|v| core_sol.get_item_info(&v).ok())
                    .collect_vec()
                    .into_iter()
                    .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                    .collect(),
            },
            rigs: core_fit
                .rigs
                .iter()
                .filter_map(|v| core_sol.get_item_info(&v).ok())
                .collect_vec()
                .into_iter()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                .collect(),
            drones: core_fit
                .drones
                .iter()
                .filter_map(|v| core_sol.get_item_info(&v).ok())
                .collect_vec()
                .into_iter()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                .collect(),
            fighters: core_fit
                .fighters
                .iter()
                .filter_map(|v| core_sol.get_item_info(&v).ok())
                .collect_vec()
                .into_iter()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                .collect(),
            fw_effects: core_fit
                .fw_effects
                .iter()
                .filter_map(|v| core_sol.get_item_info(&v).ok())
                .collect_vec()
                .into_iter()
                .map(|v| HItemInfo::mk_info(core_sol, &v, item_mode))
                .collect(),
        };
        Ok(fit)
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
