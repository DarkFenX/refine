use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::val) struct HValChargeVolumeFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    charges: HashMap<rc::ItemId, HValChargeVolumeItemInfo>,
}
impl From<&rc::val::ValChargeVolumeFail> for HValChargeVolumeFail {
    fn from(core_val_fail: &rc::val::ValChargeVolumeFail) -> Self {
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
struct HValChargeVolumeItemInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::ItemId,
    charge_volume: rc::AttrVal,
    max_volume: rc::AttrVal,
}
impl From<&rc::val::ValChargeVolumeChargeInfo> for HValChargeVolumeItemInfo {
    fn from(core_val_charge_info: &rc::val::ValChargeVolumeChargeInfo) -> Self {
        Self {
            parent_item_id: core_val_charge_info.parent_item_id,
            charge_volume: core_val_charge_info.charge_volume,
            max_volume: core_val_charge_info.max_volume,
        }
    }
}
