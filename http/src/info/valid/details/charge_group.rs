use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValChargeGroupFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::SolItemId, HValChargeGroupItemInfo>,
}
impl HValChargeGroupFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolValChargeGroupFail>> for HValChargeGroupFail {
    fn from(core_val_fails: &Vec<rc::SolValChargeGroupFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.charge_item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValChargeGroupItemInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::SolItemId,
    charge_group_id: Option<rc::EItemGrpId>,
    allowed_group_ids: Vec<rc::EItemGrpId>,
}
impl From<&rc::SolValChargeGroupFail> for HValChargeGroupItemInfo {
    fn from(core_val_fail: &rc::SolValChargeGroupFail) -> Self {
        Self {
            parent_item_id: core_val_fail.parent_item_id,
            charge_group_id: core_val_fail.charge_group_id,
            allowed_group_ids: core_val_fail.allowed_group_ids.clone(),
        }
    }
}
