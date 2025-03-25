use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValChargeSizeFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::ItemId, HValChargeSizeItemInfo>,
}
impl HValChargeSizeFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::val::ValChargeSizeFail>> for HValChargeSizeFail {
    fn from(core_val_fails: &Vec<rc::val::ValChargeSizeFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.charge_item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValChargeSizeItemInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::ItemId,
    charge_size: Option<rc::AttrVal>,
    allowed_size: rc::AttrVal,
}
impl From<&rc::val::ValChargeSizeFail> for HValChargeSizeItemInfo {
    fn from(core_val_fail: &rc::val::ValChargeSizeFail) -> Self {
        Self {
            parent_item_id: core_val_fail.parent_item_id,
            charge_size: core_val_fail.charge_size,
            allowed_size: core_val_fail.allowed_size,
        }
    }
}
