use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValChargeParentGroupFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    charges: HashMap<rc::ItemId, HValChargeParentGroupInfo>,
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

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
struct HValChargeParentGroupInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::ItemId,
    parent_group_id: rc::ItemGrpId,
    allowed_group_ids: Vec<rc::ItemGrpId>,
}
impl From<&rc::val::ValChargeParentGroupInfo> for HValChargeParentGroupInfo {
    fn from(core_val_charge_info: &rc::val::ValChargeParentGroupInfo) -> Self {
        Self {
            parent_item_id: core_val_charge_info.parent_item_id,
            parent_group_id: core_val_charge_info.parent_group_id,
            allowed_group_ids: core_val_charge_info.allowed_group_ids.clone(),
        }
    }
}
