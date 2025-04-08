use crate::sol::uad::{Uad, item::Item};

use super::{
    AutochargeInfo, BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, FwEffectInfo, ImplantInfo,
    ModuleInfo, ProjEffectInfo, RigInfo, ServiceInfo, ShipInfo, SkillInfo, StanceInfo, SubsystemInfo, SwEffectInfo,
};

pub enum ItemInfo {
    Autocharge(AutochargeInfo),
    Booster(BoosterInfo),
    Character(CharacterInfo),
    Charge(ChargeInfo),
    Drone(DroneInfo),
    Fighter(FighterInfo),
    FwEffect(FwEffectInfo),
    Implant(ImplantInfo),
    Module(ModuleInfo),
    ProjEffect(ProjEffectInfo),
    Rig(RigInfo),
    Service(ServiceInfo),
    Ship(ShipInfo),
    Skill(SkillInfo),
    Stance(StanceInfo),
    Subsystem(SubsystemInfo),
    SwEffect(SwEffectInfo),
}
impl ItemInfo {
    pub(in crate::sol) fn from_item(uad: &Uad, item: &Item) -> Self {
        match item {
            Item::Autocharge(autocharge) => ItemInfo::Autocharge(AutochargeInfo::from_autocharge(uad, autocharge)),
            Item::Booster(booster) => ItemInfo::Booster(BoosterInfo::from_booster(uad, booster)),
            Item::Character(charater) => ItemInfo::Character(CharacterInfo::from_character(uad, charater)),
            Item::Charge(charge) => ItemInfo::Charge(ChargeInfo::from_charge(uad, charge)),
            Item::Drone(drone) => ItemInfo::Drone(DroneInfo::from_drone(uad, drone)),
            Item::Fighter(fighter) => ItemInfo::Fighter(FighterInfo::from_fighter(uad, fighter)),
            Item::FwEffect(fw_effect) => ItemInfo::FwEffect(FwEffectInfo::from_fw_effect(uad, fw_effect)),
            Item::Implant(implant) => ItemInfo::Implant(ImplantInfo::from_implant(uad, implant)),
            Item::Module(module) => ItemInfo::Module(ModuleInfo::from_module(uad, module)),
            Item::ProjEffect(proj_effect) => ItemInfo::ProjEffect(ProjEffectInfo::from_proj_effect(uad, proj_effect)),
            Item::Rig(rig) => ItemInfo::Rig(RigInfo::from_rig(uad, rig)),
            Item::Service(service) => ItemInfo::Service(ServiceInfo::from_service(uad, service)),
            Item::Ship(ship) => ItemInfo::Ship(ShipInfo::from_ship(uad, ship)),
            Item::Skill(skill) => ItemInfo::Skill(SkillInfo::from_skill(uad, skill)),
            Item::Stance(stance) => ItemInfo::Stance(StanceInfo::from_stance(uad, stance)),
            Item::Subsystem(subsystem) => ItemInfo::Subsystem(SubsystemInfo::from_subsystem(uad, subsystem)),
            Item::SwEffect(sw_effect) => ItemInfo::SwEffect(SwEffectInfo::from_sw_effect(sw_effect)),
        }
    }
}
