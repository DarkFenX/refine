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
            rc::SsItemInfo::Character(core_character_info) => {
                Self::mk_from_core_character(core_ss, core_character_info, item_mode)
            }
            rc::SsItemInfo::Implant(core_implant_info) => {
                Self::mk_from_core_implant(core_ss, core_implant_info, item_mode)
            }
            rc::SsItemInfo::Ship(core_ship_info) => Self::mk_from_core_ship(core_ss, core_ship_info, item_mode),
            rc::SsItemInfo::Module(core_module_info) => Self::mk_from_core_module(core_ss, core_module_info, item_mode),
            rc::SsItemInfo::Rig(core_rig_info) => Self::mk_from_core_rig(core_ss, core_rig_info, item_mode),
            rc::SsItemInfo::Drone(core_drone_info) => Self::mk_from_core_drone(core_ss, core_drone_info, item_mode),
            rc::SsItemInfo::Charge(core_charge_info) => Self::mk_from_core_charge(core_ss, core_charge_info, item_mode),
            _ => Self::ToRemove(true),
        }
    }
    pub(crate) fn mk_from_core_character(
        core_ss: &mut rc::SolarSystem,
        core_character_info: &rc::SsCharacterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Character(HCharacterInfo::mk_info(core_ss, core_character_info, item_mode))
    }
    pub(crate) fn mk_from_core_implant(
        core_ss: &mut rc::SolarSystem,
        core_implant_info: &rc::SsImplantInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Implant(HImplantInfo::mk_info(core_ss, core_implant_info, item_mode))
    }
    pub(crate) fn mk_from_core_ship(
        core_ss: &mut rc::SolarSystem,
        core_ship_info: &rc::SsShipInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Ship(HShipInfo::mk_info(core_ss, core_ship_info, item_mode))
    }
    pub(crate) fn mk_from_core_module(
        core_ss: &mut rc::SolarSystem,
        core_module_info: &rc::SsModuleInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Module(HModuleInfo::mk_info(core_ss, core_module_info, item_mode))
    }
    pub(crate) fn mk_from_core_rig(
        core_ss: &mut rc::SolarSystem,
        core_rig_info: &rc::SsRigInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Rig(HRigInfo::mk_info(core_ss, core_rig_info, item_mode))
    }
    pub(crate) fn mk_from_core_drone(
        core_ss: &mut rc::SolarSystem,
        core_drone_info: &rc::SsDroneInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Drone(HDroneInfo::mk_info(core_ss, core_drone_info, item_mode))
    }
    pub(crate) fn mk_from_core_charge(
        core_ss: &mut rc::SolarSystem,
        core_charge_info: &rc::SsChargeInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self::Charge(HChargeInfo::mk_info(core_ss, core_charge_info, item_mode))
    }
}
