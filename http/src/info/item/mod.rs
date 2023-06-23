use character::HCharacterInfo;
use charge::HChargeInfo;
use drone::HDroneInfo;
use implant::HImplantInfo;
use module::HModuleInfo;
use rig::HRigInfo;
use ship::HShipInfo;

use crate::info::HItemInfoMode;

mod character;
mod charge;
mod drone;
mod implant;
mod module;
mod rig;
mod ship;

pub(in crate::info) trait MkItemInfo<T> {
    fn mk_info(core_ss: &mut rc::SolarSystem, source: T, item_mode: HItemInfoMode) -> HItemInfo;
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HItemInfo {
    Character(HCharacterInfo),
    Implant(HImplantInfo),
    Ship(HShipInfo),
    Module(HModuleInfo),
    Rig(HRigInfo),
    Drone(HDroneInfo),
    Charge(HChargeInfo),
    // TODO: remove when all item types are implemented
    ToRemove(bool),
}
impl HItemInfo {
    pub(crate) fn mk_from_core_item(
        core_ss: &mut rc::SolarSystem,
        core_item_info: &rc::SsItemInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match core_item_info {
            rc::SsItemInfo::Character(core_character_info) => Self::mk_info(core_ss, core_character_info, item_mode),
            rc::SsItemInfo::Implant(core_implant_info) => Self::mk_info(core_ss, core_implant_info, item_mode),
            rc::SsItemInfo::Ship(core_ship_info) => Self::mk_info(core_ss, core_ship_info, item_mode),
            rc::SsItemInfo::Module(core_module_info) => Self::mk_info(core_ss, core_module_info, item_mode),
            rc::SsItemInfo::Rig(core_rig_info) => Self::mk_info(core_ss, core_rig_info, item_mode),
            rc::SsItemInfo::Drone(core_drone_info) => Self::mk_info(core_ss, core_drone_info, item_mode),
            rc::SsItemInfo::Charge(core_charge_info) => Self::mk_info(core_ss, core_charge_info, item_mode),
            _ => Self::ToRemove(true),
        }
    }
}
impl MkItemInfo<&rc::SsCharacterInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_character_info: &rc::SsCharacterInfo,
        item_mode: HItemInfoMode,
    ) -> HItemInfo {
        Self::Character(HCharacterInfo::mk_info(core_ss, core_character_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsImplantInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_implant_info: &rc::SsImplantInfo,
        item_mode: HItemInfoMode,
    ) -> HItemInfo {
        Self::Implant(HImplantInfo::mk_info(core_ss, core_implant_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsShipInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_ship_info: &rc::SsShipInfo, item_mode: HItemInfoMode) -> HItemInfo {
        Self::Ship(HShipInfo::mk_info(core_ss, core_ship_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsModuleInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_module_info: &rc::SsModuleInfo,
        item_mode: HItemInfoMode,
    ) -> HItemInfo {
        Self::Module(HModuleInfo::mk_info(core_ss, core_module_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsRigInfo> for HItemInfo {
    fn mk_info(core_ss: &mut rc::SolarSystem, core_rig_info: &rc::SsRigInfo, item_mode: HItemInfoMode) -> HItemInfo {
        Self::Rig(HRigInfo::mk_info(core_ss, core_rig_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsDroneInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_drone_info: &rc::SsDroneInfo,
        item_mode: HItemInfoMode,
    ) -> HItemInfo {
        Self::Drone(HDroneInfo::mk_info(core_ss, core_drone_info, item_mode))
    }
}
impl MkItemInfo<&rc::SsChargeInfo> for HItemInfo {
    fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_charge_info: &rc::SsChargeInfo,
        item_mode: HItemInfoMode,
    ) -> HItemInfo {
        Self::Charge(HChargeInfo::mk_info(core_ss, core_charge_info, item_mode))
    }
}
