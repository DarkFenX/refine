use std::collections::HashMap;

use module::ModuleInfo;
use ship::ShipInfo;

use super::ItemInfoMode;

mod module;
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
        core_ss: &mut reefast::SolarSystem,
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
    Ship(ShipInfo),
    Module(ModuleInfo),
}
impl ItemInfoBasic {
    fn get_id(&self) -> reefast::ReeId {
        match self {
            Self::Ship(si) => si.item_id,
            Self::Module(mi) => mi.item_id,
        }
    }
}
impl From<&reefast::ItemInfo> for ItemInfoBasic {
    fn from(value: &reefast::ItemInfo) -> Self {
        match value {
            reefast::ItemInfo::Ship(si) => Self::Ship(si.into()),
            reefast::ItemInfo::Module(mi) => Self::Module(mi.into()),
            _ => Self::Ship(ShipInfo {
                item_id: 0,
                fit_id: 0,
                type_id: 0,
                enabled: false,
            }),
        }
    }
}
impl From<&reefast::ModuleInfo> for ItemInfoBasic {
    fn from(value: &reefast::ModuleInfo) -> Self {
        ItemInfoBasic::Module(value.into())
    }
}
impl From<&reefast::ShipInfo> for ItemInfoBasic {
    fn from(value: &reefast::ShipInfo) -> Self {
        ItemInfoBasic::Ship(value.into())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct ItemInfoFull {
    #[serde(flatten)]
    pub(crate) basic_info: ItemInfoBasic,
    pub(crate) orig_attrs: HashMap<reefast::ReeInt, reefast::ReeFloat>,
    pub(crate) dogma_attrs: HashMap<reefast::ReeInt, reefast::ReeFloat>,
}
impl ItemInfoFull {
    fn mk_info<T: Into<ItemInfoBasic>>(core_ss: &mut reefast::SolarSystem, item_identity: T) -> Self {
        let info = item_identity.into();
        let item_id = info.get_id();
        let dogma_attrs = match core_ss.get_item_dogma_attrs(&item_id) {
            Ok(attrs) => attrs,
            _ => HashMap::new(),
        };
        Self {
            basic_info: info,
            orig_attrs: HashMap::new(),
            dogma_attrs,
        }
    }
}
