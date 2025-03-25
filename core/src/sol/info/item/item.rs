use crate::sol::{SolarSystem, uad::item::Item};

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
    pub(in crate::sol) fn from_sol_item(sol_item: &Item, sol: &SolarSystem) -> Self {
        match sol_item {
            Item::Autocharge(sol_autocharge) => ItemInfo::Autocharge(sol_autocharge.into()),
            Item::Booster(sol_booster) => ItemInfo::Booster(sol.make_booster_info(sol_booster)),
            Item::Character(sol_charater) => ItemInfo::Character(sol_charater.into()),
            Item::Charge(sol_charge) => ItemInfo::Charge(sol_charge.into()),
            Item::Drone(sol_drone) => ItemInfo::Drone(sol.make_drone_info(sol_drone)),
            Item::Fighter(sol_fighter) => ItemInfo::Fighter(sol.make_fighter_info(sol_fighter)),
            Item::FwEffect(sol_fw_effect) => ItemInfo::FwEffect(sol_fw_effect.into()),
            Item::Implant(sol_implant) => ItemInfo::Implant(sol_implant.into()),
            Item::Module(sol_module) => ItemInfo::Module(sol.make_module_info(sol_module)),
            Item::ProjEffect(sol_proj_effect) => ItemInfo::ProjEffect(sol_proj_effect.into()),
            Item::Rig(sol_rig) => ItemInfo::Rig(sol_rig.into()),
            Item::Service(sol_service) => ItemInfo::Service(sol_service.into()),
            Item::Ship(sol_ship) => ItemInfo::Ship(sol_ship.into()),
            Item::Skill(sol_skill) => ItemInfo::Skill(sol_skill.into()),
            Item::Stance(sol_stance) => ItemInfo::Stance(sol_stance.into()),
            Item::Subsystem(sol_subsystem) => ItemInfo::Subsystem(sol_subsystem.into()),
            Item::SwEffect(sol_sw_effect) => ItemInfo::SwEffect(sol_sw_effect.into()),
        }
    }
}
