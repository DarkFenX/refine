use serde::Serialize;

use crate::info::{
    HItemInfoMode,
    item::{
        item_autocharge::HAutochargeInfo, item_booster::HBoosterInfo, item_character::HCharacterInfo,
        item_charge::HChargeInfo, item_drone::HDroneInfo, item_fighter::HFighterInfo, item_fw_effect::HFwEffectInfo,
        item_implant::HImplantInfo, item_module::HModuleInfo, item_proj_effect::HProjEffectInfo, item_rig::HRigInfo,
        item_service::HServiceInfo, item_ship::HShipInfo, item_skill::HSkillInfo, item_stance::HStanceInfo,
        item_subsystem::HSubsystemInfo, item_sw_effect::HSwEffectInfo,
    },
};

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HItemInfo {
    Autocharge(HAutochargeInfo),
    Booster(HBoosterInfo),
    Character(HCharacterInfo),
    Charge(HChargeInfo),
    Drone(HDroneInfo),
    Fighter(HFighterInfo),
    FwEffect(HFwEffectInfo),
    Implant(HImplantInfo),
    Module(Box<HModuleInfo>),
    ProjEffect(HProjEffectInfo),
    Rig(HRigInfo),
    Service(HServiceInfo),
    Ship(HShipInfo),
    Skill(HSkillInfo),
    Stance(HStanceInfo),
    Subsystem(HSubsystemInfo),
    SwEffect(HSwEffectInfo),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemInfo {
    pub(crate) fn from_core(core_item: &mut rc::ItemMut, item_mode: HItemInfoMode) -> Self {
        match core_item {
            rc::ItemMut::Autocharge(core_autocharge) => Self::from_core_autocharge(core_autocharge, item_mode),
            rc::ItemMut::Booster(core_booster) => Self::from_core_booster(core_booster, item_mode),
            rc::ItemMut::Character(core_character) => Self::from_core_character(core_character, item_mode),
            rc::ItemMut::Charge(core_charge) => Self::from_core_charge(core_charge, item_mode),
            rc::ItemMut::Drone(core_drone) => Self::from_core_drone(core_drone, item_mode),
            rc::ItemMut::Fighter(core_fighter) => Self::from_core_fighter(core_fighter, item_mode),
            rc::ItemMut::FwEffect(core_fw_effect) => Self::from_core_fw_effect(core_fw_effect, item_mode),
            rc::ItemMut::Implant(core_implant) => Self::from_core_implant(core_implant, item_mode),
            rc::ItemMut::Module(core_module) => Self::from_core_module(core_module, item_mode),
            rc::ItemMut::ProjEffect(core_proj_effect) => Self::from_core_proj_effect(core_proj_effect, item_mode),
            rc::ItemMut::Rig(core_rig) => Self::from_core_rig(core_rig, item_mode),
            rc::ItemMut::Service(core_service) => Self::from_core_service(core_service, item_mode),
            rc::ItemMut::Ship(core_ship) => Self::from_core_ship(core_ship, item_mode),
            rc::ItemMut::Skill(core_skill) => Self::from_core_skill(core_skill, item_mode),
            rc::ItemMut::Stance(core_stance) => Self::from_core_stance(core_stance, item_mode),
            rc::ItemMut::Subsystem(core_subsystem) => Self::from_core_subsystem(core_subsystem, item_mode),
            rc::ItemMut::SwEffect(core_sw_effect) => Self::from_core_sw_effect(core_sw_effect, item_mode),
        }
    }
    pub(crate) fn from_core_autocharge(core_autocharge: &mut rc::AutochargeMut, item_mode: HItemInfoMode) -> Self {
        Self::Autocharge(HAutochargeInfo::from_core(core_autocharge, item_mode))
    }
    pub(crate) fn from_core_booster(core_booster: &mut rc::BoosterMut, item_mode: HItemInfoMode) -> Self {
        Self::Booster(HBoosterInfo::from_core(core_booster, item_mode))
    }
    pub(crate) fn from_core_character(core_character: &mut rc::CharacterMut, item_mode: HItemInfoMode) -> Self {
        Self::Character(HCharacterInfo::from_core(core_character, item_mode))
    }
    pub(crate) fn from_core_charge(core_charge: &mut rc::ChargeMut, item_mode: HItemInfoMode) -> Self {
        Self::Charge(HChargeInfo::from_core(core_charge, item_mode))
    }
    pub(crate) fn from_core_drone(core_drone: &mut rc::DroneMut, item_mode: HItemInfoMode) -> Self {
        Self::Drone(HDroneInfo::from_core(core_drone, item_mode))
    }
    pub(crate) fn from_core_fighter(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        Self::Fighter(HFighterInfo::from_core(core_fighter, item_mode))
    }
    pub(crate) fn from_core_fw_effect(core_fw_effect: &mut rc::FwEffectMut, item_mode: HItemInfoMode) -> Self {
        Self::FwEffect(HFwEffectInfo::from_core(core_fw_effect, item_mode))
    }
    pub(crate) fn from_core_implant(core_implant: &mut rc::ImplantMut, item_mode: HItemInfoMode) -> Self {
        Self::Implant(HImplantInfo::from_core(core_implant, item_mode))
    }
    pub(crate) fn from_core_module(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        Self::Module(Box::new(HModuleInfo::from_core(core_module, item_mode)))
    }
    pub(crate) fn from_core_proj_effect(core_proj_effect: &mut rc::ProjEffectMut, item_mode: HItemInfoMode) -> Self {
        Self::ProjEffect(HProjEffectInfo::from_core(core_proj_effect, item_mode))
    }
    pub(crate) fn from_core_rig(core_rig: &mut rc::RigMut, item_mode: HItemInfoMode) -> Self {
        Self::Rig(HRigInfo::from_core(core_rig, item_mode))
    }
    pub(crate) fn from_core_service(core_service: &mut rc::ServiceMut, item_mode: HItemInfoMode) -> Self {
        Self::Service(HServiceInfo::from_core(core_service, item_mode))
    }
    pub(crate) fn from_core_ship(core_ship: &mut rc::ShipMut, item_mode: HItemInfoMode) -> Self {
        Self::Ship(HShipInfo::from_core(core_ship, item_mode))
    }
    pub(crate) fn from_core_skill(core_skill: &mut rc::SkillMut, item_mode: HItemInfoMode) -> Self {
        Self::Skill(HSkillInfo::from_core(core_skill, item_mode))
    }
    pub(crate) fn from_core_stance(core_stance: &mut rc::StanceMut, item_mode: HItemInfoMode) -> Self {
        Self::Stance(HStanceInfo::from_core(core_stance, item_mode))
    }
    pub(crate) fn from_core_subsystem(core_subsystem: &mut rc::SubsystemMut, item_mode: HItemInfoMode) -> Self {
        Self::Subsystem(HSubsystemInfo::from_core(core_subsystem, item_mode))
    }
    pub(crate) fn from_core_sw_effect(core_sw_effect: &mut rc::SwEffectMut, item_mode: HItemInfoMode) -> Self {
        Self::SwEffect(HSwEffectInfo::from_core(core_sw_effect, item_mode))
    }
}
