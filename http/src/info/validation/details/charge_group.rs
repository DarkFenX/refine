use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValChargeGroupFail {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    charges: Vec<(rc::ItemId, HValChargeGroupItemInfo)>,
}
impl From<&rc::val::ValChargeGroupFail> for HValChargeGroupFail {
    fn from(core_val_fail: &rc::val::ValChargeGroupFail) -> Self {
        Self {
            charges: core_val_fail
                .charges
                .iter()
                .map(|(charge_item_id, core_charge_info)| (*charge_item_id, core_charge_info.into()))
                .collect(),
        }
    }
}

#[serde_as]
#[derive(Serialize_tuple)]
struct HValChargeGroupItemInfo {
    #[serde_as(as = "DisplayFromStr")]
    parent_item_id: rc::ItemId,
    charge_group_id: i32,
    allowed_group_ids: Vec<i32>,
}
impl From<&rc::val::ValChargeGroupChargeInfo> for HValChargeGroupItemInfo {
    fn from(core_val_charge_info: &rc::val::ValChargeGroupChargeInfo) -> Self {
        Self {
            parent_item_id: core_val_charge_info.parent_item_id,
            charge_group_id: core_val_charge_info.charge_group_id.into_i32(),
            allowed_group_ids: core_val_charge_info
                .allowed_group_ids
                .iter()
                .map(|v| v.into_i32())
                .collect(),
        }
    }
}
