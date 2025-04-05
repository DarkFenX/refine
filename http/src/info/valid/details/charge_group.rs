use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::valid) struct HValChargeGroupFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    charges: HashMap<rc::ItemId, HValChargeGroupItemInfo>,
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

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValChargeGroupItemInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::ItemId,
    charge_group_id: rc::ItemGrpId,
    allowed_group_ids: Vec<rc::ItemGrpId>,
}
impl From<&rc::val::ValChargeGroupChargeInfo> for HValChargeGroupItemInfo {
    fn from(core_val_charge_info: &rc::val::ValChargeGroupChargeInfo) -> Self {
        Self {
            parent_item_id: core_val_charge_info.parent_item_id,
            charge_group_id: core_val_charge_info.charge_group_id,
            allowed_group_ids: core_val_charge_info.allowed_group_ids.clone(),
        }
    }
}
