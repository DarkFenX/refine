use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HChargeSizeValFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::SolItemId, HChargeSizeInfo>,
}
impl HChargeSizeValFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolChargeSizeValFail>> for HChargeSizeValFail {
    fn from(core_val_fails: &Vec<rc::SolChargeSizeValFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.charge_item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HChargeSizeInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::SolItemId,
    charge_size: Option<rc::AttrVal>,
    allowed_size: rc::AttrVal,
}
impl From<&rc::SolChargeSizeValFail> for HChargeSizeInfo {
    fn from(core_val_fail: &rc::SolChargeSizeValFail) -> Self {
        Self {
            parent_item_id: core_val_fail.parent_item_id,
            charge_size: core_val_fail.charge_size,
            allowed_size: core_val_fail.allowed_size,
        }
    }
}
