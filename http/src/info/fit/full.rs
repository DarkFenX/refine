use rc::Lender;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    info::{HItemInfo, HItemInfoMode},
    shared::HDpsProfile,
};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFitInfoFull {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::FitId,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet: Option<rc::FleetId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    character: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    skills: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    implants: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    boosters: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ship: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stance: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    subsystems: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "HModuleRacks::is_empty")]
    modules: HModuleRacks,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    rigs: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    services: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    drones: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fighters: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fw_effects: Vec<HItemInfo>,
    sec_status: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    rah_incoming_dps: Option<HDpsProfile>,
}

#[derive(Serialize)]
struct HModuleRacks {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    high: Vec<Option<HItemInfo>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    mid: Vec<Option<HItemInfo>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    low: Vec<Option<HItemInfo>>,
}
impl HModuleRacks {
    fn is_empty(&self) -> bool {
        self.high.is_empty() && self.mid.is_empty() && self.low.is_empty()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFitInfoFull {
    pub(in crate::info::fit) fn from_core(core_fit: &mut rc::FitMut, item_mode: HItemInfoMode) -> Self {
        Self {
            id: core_fit.get_fit_id(),
            fleet: core_fit.get_fleet().map(|v| v.get_fleet_id()),
            character: core_fit
                .get_character_mut()
                .map(|mut core_character| HItemInfo::from_core_character(&mut core_character, item_mode)),
            skills: core_fit
                .iter_skills_mut()
                .map_into_iter(|mut core_skill| HItemInfo::from_core_skill(&mut core_skill, item_mode))
                .collect(),
            implants: core_fit
                .iter_implants_mut()
                .map_into_iter(|mut core_implant| HItemInfo::from_core_implant(&mut core_implant, item_mode))
                .collect(),
            boosters: core_fit
                .iter_boosters_mut()
                .map_into_iter(|mut core_booster| HItemInfo::from_core_booster(&mut core_booster, item_mode))
                .collect(),
            ship: core_fit
                .get_ship_mut()
                .map(|mut core_ship| HItemInfo::from_core_ship(&mut core_ship, item_mode)),
            stance: core_fit
                .get_stance_mut()
                .map(|mut core_stance| HItemInfo::from_core_stance(&mut core_stance, item_mode)),
            subsystems: core_fit
                .iter_subsystems_mut()
                .map_into_iter(|mut core_subsystem| HItemInfo::from_core_subsystem(&mut core_subsystem, item_mode))
                .collect(),
            modules: HModuleRacks {
                high: core_fit
                    .iter_modules_mut(rc::ModRack::High)
                    .map_into_iter(|core_module| {
                        core_module.map(|mut core_module| HItemInfo::from_core_module(&mut core_module, item_mode))
                    })
                    .collect(),
                mid: core_fit
                    .iter_modules_mut(rc::ModRack::Mid)
                    .map_into_iter(|core_module| {
                        core_module.map(|mut core_module| HItemInfo::from_core_module(&mut core_module, item_mode))
                    })
                    .collect(),
                low: core_fit
                    .iter_modules_mut(rc::ModRack::Low)
                    .map_into_iter(|core_module| {
                        core_module.map(|mut core_module| HItemInfo::from_core_module(&mut core_module, item_mode))
                    })
                    .collect(),
            },
            rigs: core_fit
                .iter_rigs_mut()
                .map_into_iter(|mut core_rig| HItemInfo::from_core_rig(&mut core_rig, item_mode))
                .collect(),
            services: core_fit
                .iter_services_mut()
                .map_into_iter(|mut core_service| HItemInfo::from_core_service(&mut core_service, item_mode))
                .collect(),
            drones: core_fit
                .iter_drones_mut()
                .map_into_iter(|mut core_drone| HItemInfo::from_core_drone(&mut core_drone, item_mode))
                .collect(),
            fighters: core_fit
                .iter_fighters_mut()
                .map_into_iter(|mut core_fighter| HItemInfo::from_core_fighter(&mut core_fighter, item_mode))
                .collect(),
            fw_effects: core_fit
                .iter_fw_effects_mut()
                .map_into_iter(|mut core_fw_effect| HItemInfo::from_core_fw_effect(&mut core_fw_effect, item_mode))
                .collect(),
            sec_status: core_fit.get_sec_status().into_f64(),
            rah_incoming_dps: core_fit.get_rah_incoming_dps().map(HDpsProfile::from_core),
        }
    }
}
