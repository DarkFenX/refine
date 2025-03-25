use autocharge::HAutochargeInfo;
use booster::HBoosterInfo;
use character::HCharacterInfo;
use charge::HChargeInfo;
use drone::HDroneInfo;
use fighter::HFighterInfo;
use fw_effect::HFwEffectInfo;
use implant::HImplantInfo;
use module::HModuleInfo;
use proj_effect::HProjEffectInfo;
use rig::HRigInfo;
use service::HServiceInfo;
use ship::HShipInfo;
use skill::HSkillInfo;
use stance::HStanceInfo;
use subsystem::HSubsystemInfo;
use sw_effect::HSwEffectInfo;

use crate::info::HItemInfoMode;

mod autocharge;
mod booster;
mod character;
mod charge;
mod drone;
mod extended;
mod fighter;
mod fw_effect;
mod implant;
mod module;
mod mutation;
mod proj_effect;
mod rig;
mod service;
mod ship;
mod skill;
mod stance;
mod subsystem;
mod sw_effect;

pub(crate) trait MkItemInfo<T> {
    fn mk_info(core_sol: &mut rc::SolarSystem, source: T, item_mode: HItemInfoMode) -> HItemInfo;
}

#[derive(serde::Serialize)]
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
    Module(HModuleInfo),
    ProjEffect(HProjEffectInfo),
    Rig(HRigInfo),
    Service(HServiceInfo),
    Ship(HShipInfo),
    Skill(HSkillInfo),
    Stance(HStanceInfo),
    Subsystem(HSubsystemInfo),
    SwEffect(HSwEffectInfo),
}
impl MkItemInfo<&rc::ItemInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_item_info: &rc::ItemInfo, item_mode: HItemInfoMode) -> Self {
        match core_item_info {
            rc::ItemInfo::Autocharge(core_autocharge_info) => Self::mk_info(core_sol, core_autocharge_info, item_mode),
            rc::ItemInfo::Booster(core_booster_info) => Self::mk_info(core_sol, core_booster_info, item_mode),
            rc::ItemInfo::Character(core_character_info) => Self::mk_info(core_sol, core_character_info, item_mode),
            rc::ItemInfo::Charge(core_charge_info) => Self::mk_info(core_sol, core_charge_info, item_mode),
            rc::ItemInfo::Drone(core_drone_info) => Self::mk_info(core_sol, core_drone_info, item_mode),
            rc::ItemInfo::Fighter(core_fighter_info) => Self::mk_info(core_sol, core_fighter_info, item_mode),
            rc::ItemInfo::FwEffect(core_fw_effect_info) => Self::mk_info(core_sol, core_fw_effect_info, item_mode),
            rc::ItemInfo::Implant(core_implant_info) => Self::mk_info(core_sol, core_implant_info, item_mode),
            rc::ItemInfo::Module(core_module_info) => Self::mk_info(core_sol, core_module_info, item_mode),
            rc::ItemInfo::ProjEffect(core_proj_effect_info) => {
                Self::mk_info(core_sol, core_proj_effect_info, item_mode)
            }
            rc::ItemInfo::Rig(core_rig_info) => Self::mk_info(core_sol, core_rig_info, item_mode),
            rc::ItemInfo::Service(core_service_info) => Self::mk_info(core_sol, core_service_info, item_mode),
            rc::ItemInfo::Ship(core_ship_info) => Self::mk_info(core_sol, core_ship_info, item_mode),
            rc::ItemInfo::Skill(core_skill_info) => Self::mk_info(core_sol, core_skill_info, item_mode),
            rc::ItemInfo::Stance(core_stance_info) => Self::mk_info(core_sol, core_stance_info, item_mode),
            rc::ItemInfo::Subsystem(core_subsystem_info) => Self::mk_info(core_sol, core_subsystem_info, item_mode),
            rc::ItemInfo::SwEffect(core_sw_effect_info) => Self::mk_info(core_sol, core_sw_effect_info, item_mode),
        }
    }
}
impl MkItemInfo<&rc::AutochargeInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_autocharge_info: &rc::AutochargeInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Autocharge(HAutochargeInfo::mk_info(core_sol, core_autocharge_info, item_mode))
    }
}
impl MkItemInfo<&rc::BoosterInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_booster_info: &rc::BoosterInfo, item_mode: HItemInfoMode) -> Self {
        Self::Booster(HBoosterInfo::mk_info(core_sol, core_booster_info, item_mode))
    }
}
impl MkItemInfo<&rc::CharacterInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_character_info: &rc::CharacterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Character(HCharacterInfo::mk_info(core_sol, core_character_info, item_mode))
    }
}
impl MkItemInfo<&rc::ChargeInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_charge_info: &rc::ChargeInfo, item_mode: HItemInfoMode) -> Self {
        Self::Charge(HChargeInfo::mk_info(core_sol, core_charge_info, item_mode))
    }
}
impl MkItemInfo<&rc::DroneInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_drone_info: &rc::DroneInfo, item_mode: HItemInfoMode) -> Self {
        Self::Drone(HDroneInfo::mk_info(core_sol, core_drone_info, item_mode))
    }
}
impl MkItemInfo<&rc::FighterInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_fighter_info: &rc::FighterInfo, item_mode: HItemInfoMode) -> Self {
        Self::Fighter(HFighterInfo::mk_info(core_sol, core_fighter_info, item_mode))
    }
}
impl MkItemInfo<&rc::FwEffectInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_fw_effect_info: &rc::FwEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::FwEffect(HFwEffectInfo::mk_info(core_sol, core_fw_effect_info, item_mode))
    }
}
impl MkItemInfo<&rc::ImplantInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_implant_info: &rc::ImplantInfo, item_mode: HItemInfoMode) -> Self {
        Self::Implant(HImplantInfo::mk_info(core_sol, core_implant_info, item_mode))
    }
}
impl MkItemInfo<&rc::ModuleInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_module_info: &rc::ModuleInfo, item_mode: HItemInfoMode) -> Self {
        Self::Module(HModuleInfo::mk_info(core_sol, core_module_info, item_mode))
    }
}
impl MkItemInfo<&rc::ProjEffectInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_proj_effect_info: &rc::ProjEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::ProjEffect(HProjEffectInfo::mk_info(core_sol, core_proj_effect_info, item_mode))
    }
}
impl MkItemInfo<&rc::RigInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_rig_info: &rc::RigInfo, item_mode: HItemInfoMode) -> Self {
        Self::Rig(HRigInfo::mk_info(core_sol, core_rig_info, item_mode))
    }
}
impl MkItemInfo<&rc::ServiceInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_service_info: &rc::ServiceInfo, item_mode: HItemInfoMode) -> Self {
        Self::Service(HServiceInfo::mk_info(core_sol, core_service_info, item_mode))
    }
}
impl MkItemInfo<&rc::ShipInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_ship_info: &rc::ShipInfo, item_mode: HItemInfoMode) -> Self {
        Self::Ship(HShipInfo::mk_info(core_sol, core_ship_info, item_mode))
    }
}
impl MkItemInfo<&rc::SkillInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_skill_info: &rc::SkillInfo, item_mode: HItemInfoMode) -> Self {
        Self::Skill(HSkillInfo::mk_info(core_sol, core_skill_info, item_mode))
    }
}
impl MkItemInfo<&rc::StanceInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_stance_info: &rc::StanceInfo, item_mode: HItemInfoMode) -> Self {
        Self::Stance(HStanceInfo::mk_info(core_sol, core_stance_info, item_mode))
    }
}
impl MkItemInfo<&rc::SubsystemInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_subsystem_info: &rc::SubsystemInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Subsystem(HSubsystemInfo::mk_info(core_sol, core_subsystem_info, item_mode))
    }
}
impl MkItemInfo<&rc::SwEffectInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_sw_effect_info: &rc::SwEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::SwEffect(HSwEffectInfo::mk_info(core_sol, core_sw_effect_info, item_mode))
    }
}
