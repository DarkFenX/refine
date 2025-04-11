use crate::sol::uad::{Uad, item::UadItem};

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
    pub(in crate::sol) fn from_item(uad: &Uad, item: &UadItem) -> Self {
        match item {
            UadItem::Autocharge(autocharge) => ItemInfo::Autocharge(AutochargeInfo::from_autocharge(uad, autocharge)),
            UadItem::Booster(booster) => ItemInfo::Booster(BoosterInfo::from_booster(uad, booster)),
            UadItem::Character(charater) => ItemInfo::Character(CharacterInfo::from_character(uad, charater)),
            UadItem::Charge(charge) => ItemInfo::Charge(ChargeInfo::from_charge(uad, charge)),
            UadItem::Drone(drone) => ItemInfo::Drone(DroneInfo::from_drone(uad, drone)),
            UadItem::Fighter(fighter) => ItemInfo::Fighter(FighterInfo::from_fighter(uad, fighter)),
            UadItem::FwEffect(fw_effect) => ItemInfo::FwEffect(FwEffectInfo::from_fw_effect(uad, fw_effect)),
            UadItem::Implant(implant) => ItemInfo::Implant(ImplantInfo::from_implant(uad, implant)),
            UadItem::Module(module) => ItemInfo::Module(ModuleInfo::from_module(uad, module)),
            UadItem::ProjEffect(proj_effect) => {
                ItemInfo::ProjEffect(ProjEffectInfo::from_proj_effect(uad, proj_effect))
            }
            UadItem::Rig(rig) => ItemInfo::Rig(RigInfo::from_rig(uad, rig)),
            UadItem::Service(service) => ItemInfo::Service(ServiceInfo::from_service(uad, service)),
            UadItem::Ship(ship) => ItemInfo::Ship(ShipInfo::from_ship(uad, ship)),
            UadItem::Skill(skill) => ItemInfo::Skill(SkillInfo::from_skill(uad, skill)),
            UadItem::Stance(stance) => ItemInfo::Stance(StanceInfo::from_stance(uad, stance)),
            UadItem::Subsystem(subsystem) => ItemInfo::Subsystem(SubsystemInfo::from_subsystem(uad, subsystem)),
            UadItem::SwEffect(sw_effect) => ItemInfo::SwEffect(SwEffectInfo::from_sw_effect(sw_effect)),
        }
    }
}
