use crate::ss::{item::SsItem, SolarSystem};

use super::{
    SsBoosterInfo, SsCharacterInfo, SsChargeInfo, SsDroneInfo, SsFighterInfo, SsFwEffectInfo, SsImplantInfo,
    SsModuleInfo, SsProjEffectInfo, SsRigInfo, SsShipInfo, SsSkillInfo, SsStanceInfo, SsStructureInfo, SsSubsystemInfo,
    SsSwEffectInfo,
};

pub enum SsItemInfo {
    Booster(SsBoosterInfo),
    Character(SsCharacterInfo),
    Charge(SsChargeInfo),
    Drone(SsDroneInfo),
    Fighter(SsFighterInfo),
    FwEffect(SsFwEffectInfo),
    Implant(SsImplantInfo),
    Module(SsModuleInfo),
    Rig(SsRigInfo),
    ProjEffect(SsProjEffectInfo),
    Ship(SsShipInfo),
    Skill(SsSkillInfo),
    Stance(SsStanceInfo),
    Structure(SsStructureInfo),
    Subsystem(SsSubsystemInfo),
    SwEffect(SsSwEffectInfo),
}
impl SsItemInfo {
    pub(in crate::ss) fn from_ss_item(ss_item: &SsItem, ss: &SolarSystem) -> Self {
        match ss_item {
            SsItem::Booster(ss_booster) => SsItemInfo::Booster(ss_booster.into()),
            SsItem::Character(ss_charater) => SsItemInfo::Character(ss_charater.into()),
            SsItem::Charge(ss_charge) => SsItemInfo::Charge(ss_charge.into()),
            SsItem::Drone(ss_drone) => SsItemInfo::Drone(ss_drone.into()),
            SsItem::Fighter(ss_fighter) => SsItemInfo::Fighter(ss_fighter.into()),
            SsItem::FwEffect(ss_fw_effect) => SsItemInfo::FwEffect(ss_fw_effect.into()),
            SsItem::Implant(ss_implant) => SsItemInfo::Implant(ss_implant.into()),
            SsItem::Module(ss_module) => SsItemInfo::Module(ss.make_mod_info(ss_module)),
            SsItem::ProjEffect(ss_proj_effect) => SsItemInfo::ProjEffect(ss_proj_effect.into()),
            SsItem::Rig(ss_rig) => SsItemInfo::Rig(ss_rig.into()),
            SsItem::Ship(ss_ship) => SsItemInfo::Ship(ss_ship.into()),
            SsItem::Skill(ss_skill) => SsItemInfo::Skill(ss_skill.into()),
            SsItem::Stance(ss_stance) => SsItemInfo::Stance(ss_stance.into()),
            SsItem::Structure(ss_structure) => SsItemInfo::Structure(ss_structure.into()),
            SsItem::Subsystem(ss_subsystem) => SsItemInfo::Subsystem(ss_subsystem.into()),
            SsItem::SwEffect(ss_sw_effect) => SsItemInfo::SwEffect(ss_sw_effect.into()),
        }
    }
}
