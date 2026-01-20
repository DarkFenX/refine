use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValChargeVolumeFail {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    charges: Vec<(rc::ItemId, HValChargeVolumeItemInfo)>,
}

#[serde_as]
#[derive(Serialize_tuple)]
struct HValChargeVolumeItemInfo {
    #[serde_as(as = "DisplayFromStr")]
    parent_item_id: rc::ItemId,
    charge_volume: f64,
    max_volume: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValChargeVolumeFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValChargeVolumeFail) -> Self {
        Self {
            charges: core_val_fail
                .charges
                .into_iter()
                .map(|(charge_item_id, core_charge_info)| {
                    (charge_item_id, HValChargeVolumeItemInfo::from_core(core_charge_info))
                })
                .collect(),
        }
    }
}

impl HValChargeVolumeItemInfo {
    fn from_core(core_val_charge_info: rc::val::ValChargeVolumeChargeInfo) -> Self {
        Self {
            parent_item_id: core_val_charge_info.parent_item_id,
            charge_volume: core_val_charge_info.charge_volume.into_f64(),
            max_volume: core_val_charge_info.max_volume.into_f64(),
        }
    }
}
