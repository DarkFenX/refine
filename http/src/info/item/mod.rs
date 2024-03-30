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
use structure::HStructureInfo;
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
mod structure;
mod subsystem;
mod sw_effect;

pub(crate) trait MkItemInfo<T> {
    fn mk_info(core_ss: &mut rc::SolarSystem, source: T, item_mode: HItemInfoMode) -> HItemInfo;
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HItemInfo {
    Character(HCharacterInfo),
    Skill(HSkillInfo),
    Implant(HImplantInfo),
    Booster(HBoosterInfo),
    Ship(HShipInfo),
    Structure(HStructureInfo),
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
impl MkItemInfo<&rc::SsItemInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_item_info: &rc::SsItemInfo, item_mode: HItemInfoMode) -> Self {
        match core_item_info {
            rc::SsItemInfo::Character(core_character_info) => Self::mk_info(core_ss, core_character_info, item_mode),
            rc::SsItemInfo::Skill(core_skill_info) => Self::mk_info(core_ss, core_skill_info, item_mode),
            rc::SsItemInfo::Implant(core_implant_info) => Self::mk_info(core_ss, core_implant_info, item_mode),
            rc::SsItemInfo::Booster(core_booster_info) => Self::mk_info(core_ss, core_booster_info, item_mode),
            rc::SsItemInfo::Ship(core_ship_info) => Self::mk_info(core_ss, core_ship_info, item_mode),
            rc::SsItemInfo::Structure(core_structure_info) => Self::mk_info(core_ss, core_structure_info, item_mode),
            rc::SsItemInfo::Stance(core_stance_info) => Self::mk_info(core_ss, core_stance_info, item_mode),
            rc::SsItemInfo::Subsystem(core_subsystem_info) => Self::mk_info(core_ss, core_subsystem_info, item_mode),
            rc::SsItemInfo::Module(core_module_info) => Self::mk_info(core_ss, core_module_info, item_mode),
            rc::SsItemInfo::Rig(core_rig_info) => Self::mk_info(core_ss, core_rig_info, item_mode),
            rc::SsItemInfo::Drone(core_drone_info) => Self::mk_info(core_ss, core_drone_info, item_mode),
            rc::SsItemInfo::Fighter(core_fighter_info) => Self::mk_info(core_ss, core_fighter_info, item_mode),
            rc::SsItemInfo::Charge(core_charge_info) => Self::mk_info(core_ss, core_charge_info, item_mode),
            rc::SsItemInfo::SwEffect(core_sw_effect_info) => Self::mk_info(core_ss, core_sw_effect_info, item_mode),
            rc::SsItemInfo::FwEffect(core_fw_effect_info) => Self::mk_info(core_ss, core_fw_effect_info, item_mode),
            rc::SsItemInfo::ProjEffect(core_proj_effect_info) => {
                Self::mk_info(core_ss, core_proj_effect_info, item_mode)
            }
        }
    }
}
impl MkItemInfo<&rc::SsCharacterInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_character_info: &rc::SsCharacterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Character(HCharacterInfo::mk_info(core_ss, core_character_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsSkillInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_skill_info: &rc::SsSkillInfo, item_mode: HItemInfoMode) -> Self {
        Self::Skill(HSkillInfo::mk_info(core_ss, core_skill_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsImplantInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_implant_info: &rc::SsImplantInfo, item_mode: HItemInfoMode) -> Self {
        Self::Implant(HImplantInfo::mk_info(core_ss, core_implant_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsBoosterInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_booster_info: &rc::SsBoosterInfo, item_mode: HItemInfoMode) -> Self {
        Self::Booster(HBoosterInfo::mk_info(core_ss, core_booster_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsShipInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_ship_info: &rc::SsShipInfo, item_mode: HItemInfoMode) -> Self {
        Self::Ship(HShipInfo::mk_info(core_ss, core_ship_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsStructureInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_structure_info: &rc::SsStructureInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Structure(HStructureInfo::mk_info(core_ss, core_structure_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsStanceInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_stance_info: &rc::SsStanceInfo, item_mode: HItemInfoMode) -> Self {
        Self::Stance(HStanceInfo::mk_info(core_ss, core_stance_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsSubsystemInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_subsystem_info: &rc::SsSubsystemInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Subsystem(HSubsystemInfo::mk_info(core_ss, core_subsystem_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsModuleInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_module_info: &rc::SsModuleInfo, item_mode: HItemInfoMode) -> Self {
        Self::Module(HModuleInfo::mk_info(core_ss, core_module_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsRigInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_rig_info: &rc::SsRigInfo, item_mode: HItemInfoMode) -> Self {
        Self::Rig(HRigInfo::mk_info(core_ss, core_rig_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsDroneInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_drone_info: &rc::SsDroneInfo, item_mode: HItemInfoMode) -> Self {
        Self::Drone(HDroneInfo::mk_info(core_ss, core_drone_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsFighterInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_fighter_info: &rc::SsFighterInfo, item_mode: HItemInfoMode) -> Self {
        Self::Fighter(HFighterInfo::mk_info(core_ss, core_fighter_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsChargeInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_charge_info: &rc::SsChargeInfo, item_mode: HItemInfoMode) -> Self {
        Self::Charge(HChargeInfo::mk_info(core_ss, core_charge_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsSwEffectInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_sw_effect_info: &rc::SsSwEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::SwEffect(HSwEffectInfo::mk_info(core_ss, core_sw_effect_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsFwEffectInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_fw_effect_info: &rc::SsFwEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::FwEffect(HFwEffectInfo::mk_info(core_ss, core_fw_effect_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsProjEffectInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_proj_effect_info: &rc::SsProjEffectInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::ProjEffect(HProjEffectInfo::mk_info(core_ss, core_proj_effect_info, item_mode))
    }
}
