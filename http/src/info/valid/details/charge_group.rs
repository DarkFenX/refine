use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValChargeGroupFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::ItemId, HValChargeGroupItemInfo>,
}
impl HValChargeGroupFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::val::ValChargeGroupFail>> for HValChargeGroupFail {
    fn from(core_val_fails: &Vec<rc::val::ValChargeGroupFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.charge_item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValChargeGroupItemInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::ItemId,
    charge_group_id: Option<rc::ItemGrpId>,
    allowed_group_ids: Vec<rc::ItemGrpId>,
}
impl From<&rc::val::ValChargeGroupFail> for HValChargeGroupItemInfo {
    fn from(core_val_fail: &rc::val::ValChargeGroupFail) -> Self {
        Self {
            parent_item_id: core_val_fail.parent_item_id,
            charge_group_id: core_val_fail.charge_group_id,
            allowed_group_ids: core_val_fail.allowed_group_ids.clone(),
        }
    }
}
