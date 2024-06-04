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
use ship::HShipInfo;
use skill::HSkillInfo;
use stance::HStanceInfo;
use subsystem::HSubsystemInfo;
use sw_effect::HSwEffectInfo;

use crate::info::HItemInfoMode;

mod booster;
mod character;
mod charge;
mod drone;
mod fighter;
mod fw_effect;
mod implant;
mod module;
mod proj_effect;
mod rig;
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
    Character(HCharacterInfo),
    Skill(HSkillInfo),
    Implant(HImplantInfo),
    Booster(HBoosterInfo),
    Ship(HShipInfo),
    Stance(HStanceInfo),
    Subsystem(HSubsystemInfo),
    Module(HModuleInfo),
    Rig(HRigInfo),
    Drone(HDroneInfo),
    Fighter(HFighterInfo),
    Charge(HChargeInfo),
    SwEffect(HSwEffectInfo),
    FwEffect(HFwEffectInfo),
    ProjEffect(HProjEffectInfo),
}
impl MkItemInfo<&rc::SolItemInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_item_info: &rc::SolItemInfo, item_mode: HItemInfoMode) -> Self {
        match core_item_info {
            rc::SolItemInfo::Character(core_character_info) => Self::mk_info(core_sol, core_character_info, item_mode),
            rc::SolItemInfo::Skill(core_skill_info) => Self::mk_info(core_sol, core_skill_info, item_mode),
            rc::SolItemInfo::Implant(core_implant_info) => Self::mk_info(core_sol, core_implant_info, item_mode),
            rc::SolItemInfo::Booster(core_booster_info) => Self::mk_info(core_sol, core_booster_info, item_mode),
            rc::SolItemInfo::Ship(core_ship_info) => Self::mk_info(core_sol, core_ship_info, item_mode),
            rc::SolItemInfo::Stance(core_stance_info) => Self::mk_info(core_sol, core_stance_info, item_mode),
            rc::SolItemInfo::Subsystem(core_subsystem_info) => Self::mk_info(core_sol, core_subsystem_info, item_mode),
            rc::SolItemInfo::Module(core_module_info) => Self::mk_info(core_sol, core_module_info, item_mode),
            rc::SolItemInfo::Rig(core_rig_info) => Self::mk_info(core_sol, core_rig_info, item_mode),
            rc::SolItemInfo::Drone(core_drone_info) => Self::mk_info(core_sol, core_drone_info, item_mode),
            rc::SolItemInfo::Fighter(core_fighter_info) => Self::mk_info(core_sol, core_fighter_info, item_mode),
            rc::SolItemInfo::Charge(core_charge_info) => Self::mk_info(core_sol, core_charge_info, item_mode),
            rc::SolItemInfo::SwEffect(core_sw_effect_info) => Self::mk_info(core_sol, core_sw_effect_info, item_mode),
            rc::SolItemInfo::FwEffect(core_fw_effect_info) => Self::mk_info(core_sol, core_fw_effect_info, item_mode),
            rc::SolItemInfo::ProjEffect(core_proj_effect_info) => {
                Self::mk_info(core_sol, core_proj_effect_info, item_mode)
            }
        }
    }
}
impl MkItemInfo<&rc::SolCharacterInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_character_info: &rc::SolCharacterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Character(HCharacterInfo::mk_info(core_sol, core_character_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolSkillInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_skill_info: &rc::SolSkillInfo, item_mode: HItemInfoMode) -> Self {
        Self::Skill(HSkillInfo::mk_info(core_sol, core_skill_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolImplantInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_implant_info: &rc::SolImplantInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Implant(HImplantInfo::mk_info(core_sol, core_implant_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolBoosterInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_booster_info: &rc::SolBoosterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Booster(HBoosterInfo::mk_info(core_sol, core_booster_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolShipInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_ship_info: &rc::SolShipInfo, item_mode: HItemInfoMode) -> Self {
        Self::Ship(HShipInfo::mk_info(core_sol, core_ship_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolStanceInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_stance_info: &rc::SolStanceInfo, item_mode: HItemInfoMode) -> Self {
        Self::Stance(HStanceInfo::mk_info(core_sol, core_stance_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolSubsystemInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_subsystem_info: &rc::SolSubsystemInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Subsystem(HSubsystemInfo::mk_info(core_sol, core_subsystem_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolModuleInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_module_info: &rc::SolModuleInfo, item_mode: HItemInfoMode) -> Self {
        Self::Module(HModuleInfo::mk_info(core_sol, core_module_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolRigInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_rig_info: &rc::SolRigInfo, item_mode: HItemInfoMode) -> Self {
        Self::Rig(HRigInfo::mk_info(core_sol, core_rig_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolDroneInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_drone_info: &rc::SolDroneInfo, item_mode: HItemInfoMode) -> Self {
        Self::Drone(HDroneInfo::mk_info(core_sol, core_drone_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolFighterInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_fighter_info: &rc::SolFighterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Fighter(HFighterInfo::mk_info(core_sol, core_fighter_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolChargeInfo> for HItemInfo {
    fn mk_info(core_sol: &mut rc::SolarSystem, core_charge_info: &rc::SolChargeInfo, item_mode: HItemInfoMode) -> Self {
        Self::Charge(HChargeInfo::mk_info(core_sol, core_charge_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolSwEffectInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_sw_effect_info: &rc::SolSwEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::SwEffect(HSwEffectInfo::mk_info(core_sol, core_sw_effect_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolFwEffectInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_fw_effect_info: &rc::SolFwEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::FwEffect(HFwEffectInfo::mk_info(core_sol, core_fw_effect_info, item_mode))
    }
}
impl MkItemInfo<&rc::SolProjEffectInfo> for HItemInfo {
    fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_proj_effect_info: &rc::SolProjEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::ProjEffect(HProjEffectInfo::mk_info(core_sol, core_proj_effect_info, item_mode))
    }
}
