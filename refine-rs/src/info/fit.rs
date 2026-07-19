use rc::Lender;

use crate::{
    BoosterInfo, CharacterInfo, DpsProfile, DroneInfo, FighterInfo, FitId, FitInfoMode, FitSecStatus, FleetId,
    FwEffectInfo, ImplantInfo, ItemInfoMode, ModuleInfo, RigInfo, ServiceInfo, ShipInfo, SkillInfo, StanceInfo,
    SubsystemInfo,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FitInfo {
    pub id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<FitInfoExt>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FitInfoExt {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fleet_id: Option<FleetId>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub character: Option<CharacterInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub skills: Vec<SkillInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub implants: Vec<ImplantInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub boosters: Vec<BoosterInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ship: Option<ShipInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub stance: Option<StanceInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub subsystems: Vec<SubsystemInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ModuleRacks::is_empty"))]
    pub modules: ModuleRacks,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub rigs: Vec<RigInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub services: Vec<ServiceInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub drones: Vec<DroneInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fighters: Vec<FighterInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fw_effects: Vec<FwEffectInfo>,
    pub sec_status: FitSecStatus,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub rah_incoming_dps: Option<DpsProfile>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ModuleRacks {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub high: Vec<Option<ModuleInfo>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub mid: Vec<Option<ModuleInfo>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
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
