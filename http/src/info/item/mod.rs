use item_autocharge::HAutochargeInfo;
use item_booster::HBoosterInfo;
use item_character::HCharacterInfo;
use item_charge::HChargeInfo;
use item_drone::HDroneInfo;
use item_fighter::HFighterInfo;
use item_fw_effect::HFwEffectInfo;
use item_implant::HImplantInfo;
use item_module::HModuleInfo;
use item_proj_effect::HProjEffectInfo;
use item_rig::HRigInfo;
use item_service::HServiceInfo;
use item_ship::HShipInfo;
use item_skill::HSkillInfo;
use item_stance::HStanceInfo;
use item_subsystem::HSubsystemInfo;
use item_sw_effect::HSwEffectInfo;

use crate::info::HItemInfoMode;

mod extended;
mod item_autocharge;
mod item_booster;
mod item_character;
mod item_charge;
mod item_drone;
mod item_fighter;
mod item_fw_effect;
mod item_implant;
mod item_module;
mod item_proj_effect;
mod item_rig;
mod item_service;
mod item_ship;
mod item_skill;
mod item_stance;
mod item_subsystem;
mod item_sw_effect;
mod mutation;
mod proj;

pub(crate) trait MkItemInfo<T> {
    fn mk_info(source: T, item_mode: HItemInfoMode) -> HItemInfo;
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
impl MkItemInfo<&mut rc::ItemMut<'_>> for HItemInfo {
    fn mk_info(core_item: &mut rc::ItemMut, item_mode: HItemInfoMode) -> Self {
        match core_item {
            rc::ItemMut::Autocharge(core_autocharge) => Self::mk_info(core_autocharge, item_mode),
            rc::ItemMut::Booster(core_booster) => Self::mk_info(core_booster, item_mode),
            rc::ItemMut::Character(core_character) => Self::mk_info(core_character, item_mode),
            rc::ItemMut::Charge(core_charge) => Self::mk_info(core_charge, item_mode),
            rc::ItemMut::Drone(core_drone) => Self::mk_info(core_drone, item_mode),
            rc::ItemMut::Fighter(core_fighter) => Self::mk_info(core_fighter, item_mode),
            rc::ItemMut::FwEffect(core_fw_effect) => Self::mk_info(core_fw_effect, item_mode),
            rc::ItemMut::Implant(core_implant) => Self::mk_info(core_implant, item_mode),
            rc::ItemMut::Module(core_module) => Self::mk_info(core_module, item_mode),
            rc::ItemMut::ProjEffect(core_proj_effect) => Self::mk_info(core_proj_effect, item_mode),
            rc::ItemMut::Rig(core_rig) => Self::mk_info(core_rig, item_mode),
            rc::ItemMut::Service(core_service) => Self::mk_info(core_service, item_mode),
            rc::ItemMut::Ship(core_ship) => Self::mk_info(core_ship, item_mode),
            rc::ItemMut::Skill(core_skill) => Self::mk_info(core_skill, item_mode),
            rc::ItemMut::Stance(core_stance) => Self::mk_info(core_stance, item_mode),
            rc::ItemMut::Subsystem(core_subsystem) => Self::mk_info(core_subsystem, item_mode),
            rc::ItemMut::SwEffect(core_sw_effect) => Self::mk_info(core_sw_effect, item_mode),
        }
    }
}
impl MkItemInfo<&mut rc::AutochargeMut<'_>> for HItemInfo {
    fn mk_info(core_autocharge: &mut rc::AutochargeMut, item_mode: HItemInfoMode) -> Self {
        Self::Autocharge(HAutochargeInfo::mk_info(core_autocharge, item_mode))
    }
}
impl MkItemInfo<&mut rc::BoosterMut<'_>> for HItemInfo {
    fn mk_info(core_booster: &mut rc::BoosterMut, item_mode: HItemInfoMode) -> Self {
        Self::Booster(HBoosterInfo::mk_info(core_booster, item_mode))
    }
}
impl MkItemInfo<&mut rc::CharacterMut<'_>> for HItemInfo {
    fn mk_info(core_character: &mut rc::CharacterMut, item_mode: HItemInfoMode) -> Self {
        Self::Character(HCharacterInfo::mk_info(core_character, item_mode))
    }
}
impl MkItemInfo<&mut rc::ChargeMut<'_>> for HItemInfo {
    fn mk_info(core_charge: &mut rc::ChargeMut, item_mode: HItemInfoMode) -> Self {
        Self::Charge(HChargeInfo::mk_info(core_charge, item_mode))
    }
}
impl MkItemInfo<&mut rc::DroneMut<'_>> for HItemInfo {
    fn mk_info(core_drone: &mut rc::DroneMut, item_mode: HItemInfoMode) -> Self {
        Self::Drone(HDroneInfo::mk_info(core_drone, item_mode))
    }
}
impl MkItemInfo<&mut rc::FighterMut<'_>> for HItemInfo {
    fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        Self::Fighter(HFighterInfo::mk_info(core_fighter, item_mode))
    }
}
impl MkItemInfo<&mut rc::FwEffectMut<'_>> for HItemInfo {
    fn mk_info(core_fw_effect: &mut rc::FwEffectMut, item_mode: HItemInfoMode) -> Self {
        Self::FwEffect(HFwEffectInfo::mk_info(core_fw_effect, item_mode))
    }
}
impl MkItemInfo<&mut rc::ImplantMut<'_>> for HItemInfo {
    fn mk_info(core_implant: &mut rc::ImplantMut, item_mode: HItemInfoMode) -> Self {
        Self::Implant(HImplantInfo::mk_info(core_implant, item_mode))
    }
}
impl MkItemInfo<&mut rc::ModuleMut<'_>> for HItemInfo {
    fn mk_info(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        Self::Module(HModuleInfo::mk_info(core_module, item_mode))
    }
}
impl MkItemInfo<&mut rc::ProjEffectMut<'_>> for HItemInfo {
    fn mk_info(core_proj_effect: &mut rc::ProjEffectMut, item_mode: HItemInfoMode) -> Self {
        Self::ProjEffect(HProjEffectInfo::mk_info(core_proj_effect, item_mode))
    }
}
impl MkItemInfo<&mut rc::RigMut<'_>> for HItemInfo {
    fn mk_info(core_rig: &mut rc::RigMut, item_mode: HItemInfoMode) -> Self {
        Self::Rig(HRigInfo::mk_info(core_rig, item_mode))
    }
}
impl MkItemInfo<&mut rc::ServiceMut<'_>> for HItemInfo {
    fn mk_info(core_service: &mut rc::ServiceMut, item_mode: HItemInfoMode) -> Self {
        Self::Service(HServiceInfo::mk_info(core_service, item_mode))
    }
}
impl MkItemInfo<&mut rc::ShipMut<'_>> for HItemInfo {
    fn mk_info(core_ship: &mut rc::ShipMut, item_mode: HItemInfoMode) -> Self {
        Self::Ship(HShipInfo::mk_info(core_ship, item_mode))
    }
}
impl MkItemInfo<&mut rc::SkillMut<'_>> for HItemInfo {
    fn mk_info(core_skill: &mut rc::SkillMut, item_mode: HItemInfoMode) -> Self {
        Self::Skill(HSkillInfo::mk_info(core_skill, item_mode))
    }
}
impl MkItemInfo<&mut rc::StanceMut<'_>> for HItemInfo {
    fn mk_info(core_stance: &mut rc::StanceMut, item_mode: HItemInfoMode) -> Self {
        Self::Stance(HStanceInfo::mk_info(core_stance, item_mode))
    }
}
impl MkItemInfo<&mut rc::SubsystemMut<'_>> for HItemInfo {
    fn mk_info(core_subsystem: &mut rc::SubsystemMut, item_mode: HItemInfoMode) -> Self {
        Self::Subsystem(HSubsystemInfo::mk_info(core_subsystem, item_mode))
    }
}
impl MkItemInfo<&mut rc::SwEffectMut<'_>> for HItemInfo {
    fn mk_info(core_sw_effect: &mut rc::SwEffectMut, item_mode: HItemInfoMode) -> Self {
        Self::SwEffect(HSwEffectInfo::mk_info(core_sw_effect, item_mode))
    }
}
