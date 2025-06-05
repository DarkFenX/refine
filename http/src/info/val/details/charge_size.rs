use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::val) struct HValChargeSizeFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    charges: HashMap<rc::ItemId, HValChargeSizeItemInfo>,
}
impl From<&rc::val::ValChargeSizeFail> for HValChargeSizeFail {
    fn from(core_val_fail: &rc::val::ValChargeSizeFail) -> Self {
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
struct HValChargeSizeItemInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    parent_item_id: rc::ItemId,
    charge_size: Option<rc::AttrVal>,
    allowed_size: rc::AttrVal,
}
impl From<&rc::val::ValChargeSizeChargeInfo> for HValChargeSizeItemInfo {
    fn from(core_val_charge_info: &rc::val::ValChargeSizeChargeInfo) -> Self {
        Self {
            parent_item_id: core_val_charge_info.parent_item_id,
            charge_size: core_val_charge_info.charge_size,
            allowed_size: core_val_charge_info.allowed_size,
        }
    }
}
