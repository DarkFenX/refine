use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::validation) struct HValShipLimitFail {
    ship_type_id: Option<rc::ItemTypeId>,
    ship_group_id: Option<rc::ItemGrpId>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    mismatches: HashMap<rc::ItemId, HValShipLimitItemInfo>,
}
impl From<&rc::val::ValShipLimitFail> for HValShipLimitFail {
    fn from(core_val_fail: &rc::val::ValShipLimitFail) -> Self {
        Self {
            ship_type_id: core_val_fail.ship_type_id,
            ship_group_id: core_val_fail.ship_group_id,
            mismatches: core_val_fail
                .items
                .iter()
                .map(|(item_id, item_info)| (*item_id, item_info.into()))
                .collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
struct HValShipLimitItemInfo {
    allowed_type_ids: Vec<rc::ItemTypeId>,
    allowed_group_ids: Vec<rc::ItemGrpId>,
}
impl From<&rc::val::ValShipLimitItemInfo> for HValShipLimitItemInfo {
    fn from(core_item_info: &rc::val::ValShipLimitItemInfo) -> Self {
        Self {
            allowed_type_ids: core_item_info.allowed_type_ids.clone(),
            allowed_group_ids: core_item_info.allowed_group_ids.clone(),
        }
    }
}
