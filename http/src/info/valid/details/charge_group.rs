use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HChargeGroupValFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::SolItemId, HChargeGroupInfo>,
}
impl HChargeGroupValFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolChargeGroupValFail>> for HChargeGroupValFail {
    fn from(core_val_fails: &Vec<rc::SolChargeGroupValFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.charge_item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HChargeGroupInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::SolItemId,
    charge_group_id: rc::EItemGrpId,
    allowed_group_ids: Vec<rc::EItemGrpId>,
}
impl From<&rc::SolChargeGroupValFail> for HChargeGroupInfo {
    fn from(core_val_fail: &rc::SolChargeGroupValFail) -> Self {
        Self {
            parent_item_id: core_val_fail.parent_item_id,
            charge_group_id: core_val_fail.charge_group_id,
            allowed_group_ids: core_val_fail.allowed_group_ids.clone(),
        }
    }
}
