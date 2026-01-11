use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValChargeSizeFail {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    charges: Vec<(rc::ItemId, HValChargeSizeItemInfo)>,
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

#[serde_as]
#[derive(Serialize_tuple)]
struct HValChargeSizeItemInfo {
    #[serde_as(as = "DisplayFromStr")]
    parent_item_id: rc::ItemId,
    charge_size: Option<f64>,
    allowed_size: f64,
}
impl From<&rc::val::ValChargeSizeChargeInfo> for HValChargeSizeItemInfo {
    fn from(core_val_charge_info: &rc::val::ValChargeSizeChargeInfo) -> Self {
        Self {
            parent_item_id: core_val_charge_info.parent_item_id,
            charge_size: core_val_charge_info.charge_size.map(|v| v.into_f64()),
            allowed_size: core_val_charge_info.allowed_size.into_f64(),
        }
    }
}
