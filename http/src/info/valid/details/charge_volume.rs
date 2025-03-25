use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValChargeVolumeFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::ItemId, HValChargeVolumeItemInfo>,
}
impl HValChargeVolumeFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::val::ValChargeVolumeFail>> for HValChargeVolumeFail {
    fn from(core_val_fails: &Vec<rc::val::ValChargeVolumeFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.charge_item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValChargeVolumeItemInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::ItemId,
    charge_volume: rc::AttrVal,
    max_volume: rc::AttrVal,
}
impl From<&rc::val::ValChargeVolumeFail> for HValChargeVolumeItemInfo {
    fn from(core_val_fail: &rc::val::ValChargeVolumeFail) -> Self {
        Self {
            parent_item_id: core_val_fail.parent_item_id,
            charge_volume: core_val_fail.charge_volume,
            max_volume: core_val_fail.max_volume,
        }
    }
}
