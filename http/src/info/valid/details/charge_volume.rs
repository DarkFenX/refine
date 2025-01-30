use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HChargeVolumeValFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::SolItemId, HChargeVolumeInfo>,
}
impl HChargeVolumeValFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolChargeVolumeValFail>> for HChargeVolumeValFail {
    fn from(core_val_fails: &Vec<rc::SolChargeVolumeValFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.charge_item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HChargeVolumeInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::SolItemId,
    charge_volume: rc::AttrVal,
    max_volume: rc::AttrVal,
}
impl From<&rc::SolChargeVolumeValFail> for HChargeVolumeInfo {
    fn from(core_val_fail: &rc::SolChargeVolumeValFail) -> Self {
        Self {
            parent_item_id: core_val_fail.parent_item_id,
            charge_volume: core_val_fail.charge_volume,
            max_volume: core_val_fail.max_volume,
        }
    }
}
