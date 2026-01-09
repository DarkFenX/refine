use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValShipLimitFail {
    ship_type_id: Option<i32>,
    ship_group_id: Option<i32>,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    mismatches: Vec<(rc::ItemId, HValShipLimitItemInfo)>,
}
impl From<&rc::val::ValShipLimitFail> for HValShipLimitFail {
    fn from(core_val_fail: &rc::val::ValShipLimitFail) -> Self {
        Self {
            ship_type_id: core_val_fail.ship_type_id.map(|v| v.into_i32()),
            ship_group_id: core_val_fail.ship_group_id.map(|v| v.into_i32()),
            mismatches: core_val_fail
                .items
                .iter()
                .map(|(item_id, item_info)| (*item_id, item_info.into()))
                .collect(),
        }
    }
}

#[derive(Serialize_tuple)]
struct HValShipLimitItemInfo {
    allowed_type_ids: Vec<i32>,
    allowed_group_ids: Vec<i32>,
}
impl From<&rc::val::ValShipLimitItemInfo> for HValShipLimitItemInfo {
    fn from(core_item_info: &rc::val::ValShipLimitItemInfo) -> Self {
        Self {
            allowed_type_ids: core_item_info.allowed_type_ids.iter().map(|v| v.into_i32()).collect(),
            allowed_group_ids: core_item_info.allowed_group_ids.iter().map(|v| v.into_i32()).collect(),
        }
    }
}
