use std::collections::HashMap;

use character::HCharacterInfo;
use charge::HChargeInfo;
use drone::HDroneInfo;
use implant::HImplantInfo;
use module::HModuleInfo;
use rig::HRigInfo;
use ship::HShipInfo;

use crate::info::{HAttrVal, HItemInfoMode};

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
    Id(String),
    Basic(HItemInfoBasic),
    Full(HItemInfoFull),
}
impl HItemInfo {
    pub(crate) fn mk_info<T: Into<HItemInfoBasic>>(
        core_ss: &mut rc::SolarSystem,
        item_identity: T,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => {
                let info = item_identity.into();
                Self::Id(info.get_id().to_string())
            }
            HItemInfoMode::Basic => Self::Basic(item_identity.into()),
            HItemInfoMode::Full => Self::Full(HItemInfoFull::mk_info(core_ss, item_identity)),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HItemInfoBasic {
    Character(HCharacterInfo),
    Implant(HImplantInfo),
    Ship(HShipInfo),
    Module(HModuleInfo),
    Rig(HRigInfo),
    Drone(HDroneInfo),
    Charge(HChargeInfo),
}
impl HItemInfoBasic {
    fn get_id(&self) -> rc::ReeId {
        match self {
            Self::Character(info) => info.id,
            Self::Implant(info) => info.id,
            Self::Ship(info) => info.id,
            Self::Module(info) => info.id,
            Self::Rig(info) => info.id,
            Self::Drone(info) => info.id,
            Self::Charge(info) => info.id,
        }
    }
}
impl From<&rc::SsItemInfo> for HItemInfoBasic {
    fn from(ss_item_info: &rc::SsItemInfo) -> Self {
        match ss_item_info {
            rc::SsItemInfo::Character(info) => Self::Character(info.into()),
            rc::SsItemInfo::Implant(info) => Self::Implant(info.into()),
            rc::SsItemInfo::Ship(info) => Self::Ship(info.into()),
            rc::SsItemInfo::Module(info) => Self::Module(info.into()),
            rc::SsItemInfo::Rig(info) => Self::Rig(info.into()),
            rc::SsItemInfo::Drone(info) => Self::Drone(info.into()),
            rc::SsItemInfo::Charge(info) => Self::Charge(info.into()),
            // TODO: remove after all conversions were added
            _ => Self::Ship(HShipInfo {
                id: 999999,
                fit_id: 666666,
                type_id: 333333,
                enabled: false,
            }),
        }
    }
}
impl From<&rc::SsModuleInfo> for HItemInfoBasic {
    fn from(ss_module_info: &rc::SsModuleInfo) -> Self {
        HItemInfoBasic::Module(ss_module_info.into())
    }
}
impl From<&rc::SsShipInfo> for HItemInfoBasic {
    fn from(ss_ship_info: &rc::SsShipInfo) -> Self {
        HItemInfoBasic::Ship(ss_ship_info.into())
    }
}

#[derive(serde::Serialize)]
pub(crate) struct HItemInfoFull {
    #[serde(flatten)]
    pub(crate) basic_info: HItemInfoBasic,
    pub(crate) attr_vals: HashMap<rc::ReeInt, HAttrVal>,
}
impl HItemInfoFull {
    fn mk_info<T: Into<HItemInfoBasic>>(core_ss: &mut rc::SolarSystem, item_identity: T) -> Self {
        let h_info = item_identity.into();
        let item_id = h_info.get_id();
        let attrs = match core_ss.get_item_attrs(&item_id) {
            Ok(attrs) => attrs.into_iter().map(|(k, v)| (k, HAttrVal::from(&v))).collect(),
            _ => HashMap::new(),
        };
        Self {
            basic_info: h_info,
            attr_vals: attrs,
        }
    }
}
