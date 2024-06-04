use crate::sol::{item::SolItem, SolarSystem};

use super::{
    SolBoosterInfo, SolCharacterInfo, SolChargeInfo, SolDroneInfo, SolFighterInfo, SolFwEffectInfo, SolImplantInfo,
    SolModuleInfo, SolProjEffectInfo, SolRigInfo, SolShipInfo, SolSkillInfo, SolStanceInfo, SolSubsystemInfo,
    SolSwEffectInfo,
};

pub enum SolItemInfo {
    Booster(SolBoosterInfo),
    Character(SolCharacterInfo),
    Charge(SolChargeInfo),
    Drone(SolDroneInfo),
    Fighter(SolFighterInfo),
    FwEffect(SolFwEffectInfo),
    Implant(SolImplantInfo),
    Module(SolModuleInfo),
    Rig(SolRigInfo),
    ProjEffect(SolProjEffectInfo),
    Ship(SolShipInfo),
    Skill(SolSkillInfo),
    Stance(SolStanceInfo),
    Subsystem(SolSubsystemInfo),
    SwEffect(SolSwEffectInfo),
}
impl SolItemInfo {
    pub(in crate::sol) fn from_sol_item(sol_item: &SolItem, sol: &SolarSystem) -> Self {
        match sol_item {
            SolItem::Booster(sol_booster) => SolItemInfo::Booster(sol_booster.into()),
            SolItem::Character(sol_charater) => SolItemInfo::Character(sol_charater.into()),
            SolItem::Charge(sol_charge) => SolItemInfo::Charge(sol_charge.into()),
            SolItem::Drone(sol_drone) => SolItemInfo::Drone(sol_drone.into()),
            SolItem::Fighter(sol_fighter) => SolItemInfo::Fighter(sol_fighter.into()),
            SolItem::FwEffect(sol_fw_effect) => SolItemInfo::FwEffect(sol_fw_effect.into()),
            SolItem::Implant(sol_implant) => SolItemInfo::Implant(sol_implant.into()),
            SolItem::Module(sol_module) => SolItemInfo::Module(sol.make_mod_info(sol_module)),
            SolItem::ProjEffect(sol_proj_effect) => SolItemInfo::ProjEffect(sol_proj_effect.into()),
            SolItem::Rig(sol_rig) => SolItemInfo::Rig(sol_rig.into()),
            SolItem::Ship(sol_ship) => SolItemInfo::Ship(sol_ship.into()),
            SolItem::Skill(sol_skill) => SolItemInfo::Skill(sol_skill.into()),
            SolItem::Stance(sol_stance) => SolItemInfo::Stance(sol_stance.into()),
            SolItem::Subsystem(sol_subsystem) => SolItemInfo::Subsystem(sol_subsystem.into()),
            SolItem::SwEffect(sol_sw_effect) => SolItemInfo::SwEffect(sol_sw_effect.into()),
        }
    }
}
