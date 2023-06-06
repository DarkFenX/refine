use std::collections::HashMap;

use implant::ImplantInfo;
use module::ModuleInfo;
use rig::RigInfo;
use ship::ShipInfo;

use crate::info::{AttrValInfo, ItemInfoMode};

mod implant;
mod module;
mod rig;
mod ship;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum ItemInfo {
    IdOnly(String),
    Basic(ItemInfoBasic),
    Full(ItemInfoFull),
}
impl ItemInfo {
    pub(crate) fn mk_info<T: Into<ItemInfoBasic>>(
        core_ss: &mut rc::SolarSystem,
        item_identity: T,
        item_mode: ItemInfoMode,
    ) -> Self {
        match item_mode {
            ItemInfoMode::IdOnly => {
                let info = item_identity.into();
                Self::IdOnly(info.get_id().to_string())
            }
            ItemInfoMode::Basic => Self::Basic(item_identity.into()),
            ItemInfoMode::Full => Self::Full(ItemInfoFull::mk_info(core_ss, item_identity)),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum ItemInfoBasic {
    Implant(ImplantInfo),
    Ship(ShipInfo),
    Module(ModuleInfo),
    Rig(RigInfo),
}
impl ItemInfoBasic {
    fn get_id(&self) -> rc::ReeId {
        match self {
            Self::Implant(info) => info.id,
            Self::Ship(info) => info.id,
            Self::Module(info) => info.id,
            Self::Rig(info) => info.id,
        }
    }
}
impl From<&rc::SsItemInfo> for ItemInfoBasic {
    fn from(value: &rc::SsItemInfo) -> Self {
        match value {
            rc::SsItemInfo::Implant(info) => Self::Implant(info.into()),
            rc::SsItemInfo::Ship(info) => Self::Ship(info.into()),
            rc::SsItemInfo::Module(info) => Self::Module(info.into()),
            rc::SsItemInfo::Rig(info) => Self::Rig(info.into()),
            _ => Self::Ship(ShipInfo {
                id: 999999,
                fit_id: 666666,
                type_id: 333333,
                enabled: false,
            }),
        }
    }
}
impl From<&rc::SsModuleInfo> for ItemInfoBasic {
    fn from(value: &rc::SsModuleInfo) -> Self {
        ItemInfoBasic::Module(value.into())
    }
}
impl From<&rc::SsShipInfo> for ItemInfoBasic {
    fn from(value: &rc::SsShipInfo) -> Self {
        ItemInfoBasic::Ship(value.into())
    }
}

#[derive(serde::Serialize)]
pub(crate) struct ItemInfoFull {
    #[serde(flatten)]
    pub(crate) basic_info: ItemInfoBasic,
    pub(crate) attr_vals: HashMap<rc::ReeInt, AttrValInfo>,
}
impl ItemInfoFull {
    fn mk_info<T: Into<ItemInfoBasic>>(core_ss: &mut rc::SolarSystem, item_identity: T) -> Self {
        let info = item_identity.into();
        let item_id = info.get_id();
        let attrs = match core_ss.get_item_attrs(&item_id) {
            Ok(attrs) => attrs.into_iter().map(|(k, v)| (k, AttrValInfo::from(&v))).collect(),
            _ => HashMap::new(),
        };
        Self {
            basic_info: info,
            attr_vals: attrs,
        }
    }
}
