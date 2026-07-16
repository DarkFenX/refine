use rc::Lender;

use crate::{
    BoosterInfo, CharacterInfo, DroneInfo, FighterInfo, FitInfoMode, FwEffectInfo, ImplantInfo, ItemInfoMode,
    ModuleInfo, RigInfo, ServiceInfo, ShipInfo, SkillInfo, StanceInfo, SubsystemInfo,
};

pub struct FitInfo {
    pub id: rc::FitId,
    pub extended: Option<FitInfoExt>,
}

pub struct FitInfoExt {
    pub fleet_id: Option<rc::FleetId>,
    pub character: Option<CharacterInfo>,
    pub skills: Vec<SkillInfo>,
    pub implants: Vec<ImplantInfo>,
    pub boosters: Vec<BoosterInfo>,
    pub ship: Option<ShipInfo>,
    pub stance: Option<StanceInfo>,
    pub subsystems: Vec<SubsystemInfo>,
    pub modules: ModuleRacks,
    pub rigs: Vec<RigInfo>,
    pub services: Vec<ServiceInfo>,
    pub drones: Vec<DroneInfo>,
    pub fighters: Vec<FighterInfo>,
    pub fw_effects: Vec<FwEffectInfo>,
    pub sec_status: rc::FitSecStatus,
    pub rah_incoming_dps: Option<rc::DpsProfile>,
}

pub struct ModuleRacks {
    pub high: Vec<Option<ModuleInfo>>,
    pub mid: Vec<Option<ModuleInfo>>,
    pub low: Vec<Option<ModuleInfo>>,
}
impl ModuleRacks {
    pub fn is_empty(&self) -> bool {
        self.high.is_empty() && self.mid.is_empty() && self.low.is_empty()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitInfo {
    pub(crate) fn from_core(core_fit: &mut rc::FitMut, fit_mode: FitInfoMode, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_fit.get_fit_id(),
            extended: match fit_mode {
                FitInfoMode::Id => None,
                FitInfoMode::Full => Some(FitInfoExt {
                    fleet_id: core_fit.get_fleet().map(|v| v.get_fleet_id()),
                    character: core_fit
                        .get_character_mut()
                        .map(|mut core_character| CharacterInfo::from_core(&mut core_character, item_mode)),
                    skills: core_fit
                        .iter_skills_mut()
                        .map_into_iter(|mut core_skill| SkillInfo::from_core(&mut core_skill, item_mode))
                        .collect(),
                    implants: core_fit
                        .iter_implants_mut()
                        .map_into_iter(|mut core_implant| ImplantInfo::from_core(&mut core_implant, item_mode))
                        .collect(),
                    boosters: core_fit
                        .iter_boosters_mut()
                        .map_into_iter(|mut core_booster| BoosterInfo::from_core(&mut core_booster, item_mode))
                        .collect(),
                    ship: core_fit
                        .get_ship_mut()
                        .map(|mut core_ship| ShipInfo::from_core(&mut core_ship, item_mode)),
                    stance: core_fit
                        .get_stance_mut()
                        .map(|mut core_stance| StanceInfo::from_core(&mut core_stance, item_mode)),
                    subsystems: core_fit
                        .iter_subsystems_mut()
                        .map_into_iter(|mut core_subsystem| SubsystemInfo::from_core(&mut core_subsystem, item_mode))
                        .collect(),
                    modules: ModuleRacks {
                        high: core_fit
                            .iter_modules_mut(rc::ModRack::High)
                            .map_into_iter(|core_module| {
                                core_module.map(|mut core_module| ModuleInfo::from_core(&mut core_module, item_mode))
                            })
                            .collect(),
                        mid: core_fit
                            .iter_modules_mut(rc::ModRack::Mid)
                            .map_into_iter(|core_module| {
                                core_module.map(|mut core_module| ModuleInfo::from_core(&mut core_module, item_mode))
                            })
                            .collect(),
                        low: core_fit
                            .iter_modules_mut(rc::ModRack::Low)
                            .map_into_iter(|core_module| {
                                core_module.map(|mut core_module| ModuleInfo::from_core(&mut core_module, item_mode))
                            })
                            .collect(),
                    },
                    rigs: core_fit
                        .iter_rigs_mut()
                        .map_into_iter(|mut core_rig| RigInfo::from_core(&mut core_rig, item_mode))
                        .collect(),
                    services: core_fit
                        .iter_services_mut()
                        .map_into_iter(|mut core_service| ServiceInfo::from_core(&mut core_service, item_mode))
                        .collect(),
                    drones: core_fit
                        .iter_drones_mut()
                        .map_into_iter(|mut core_drone| DroneInfo::from_core(&mut core_drone, item_mode))
                        .collect(),
                    fighters: core_fit
                        .iter_fighters_mut()
                        .map_into_iter(|mut core_fighter| FighterInfo::from_core(&mut core_fighter, item_mode))
                        .collect(),
                    fw_effects: core_fit
                        .iter_fw_effects_mut()
                        .map_into_iter(|mut core_fw_effect| FwEffectInfo::from_core(&mut core_fw_effect, item_mode))
                        .collect(),
                    sec_status: core_fit.get_sec_status(),
                    rah_incoming_dps: core_fit.get_rah_incoming_dps(),
                }),
            },
        }
    }
}
