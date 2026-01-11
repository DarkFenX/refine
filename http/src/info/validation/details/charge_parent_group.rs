use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValChargeParentGroupFail {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    charges: Vec<(rc::ItemId, HValChargeParentGroupInfo)>,
}
impl From<&rc::val::ValChargeParentGroupFail> for HValChargeParentGroupFail {
    fn from(core_val_fail: &rc::val::ValChargeParentGroupFail) -> Self {
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
struct HValChargeParentGroupInfo {
    #[serde_as(as = "DisplayFromStr")]
    parent_item_id: rc::ItemId,
    parent_group_id: i32,
    allowed_group_ids: Vec<i32>,
}
impl From<&rc::val::ValChargeParentGroupInfo> for HValChargeParentGroupInfo {
    fn from(core_val_charge_info: &rc::val::ValChargeParentGroupInfo) -> Self {
        Self {
            parent_item_id: core_val_charge_info.parent_item_id,
            parent_group_id: core_val_charge_info.parent_group_id.into_i32(),
            allowed_group_ids: core_val_charge_info
                .allowed_group_ids
                .iter()
                .map(|v| v.into_i32())
                .collect(),
        }
    }
}
