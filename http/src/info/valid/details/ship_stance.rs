#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValShipStanceFail {
    ship_id: rc::SolItemId,
    stance: Option<HValShipStanceItemInfo>,
    stance_type_ids: Vec<rc::EItemId>,
}
impl From<&rc::SolValShipStanceFail> for HValShipStanceFail {
    fn from(core_val_fail: &rc::SolValShipStanceFail) -> Self {
        Self {
            ship_id: core_val_fail.ship_id,
            stance: core_val_fail.stance.as_ref().map(|v| v.into()),
            stance_type_ids: core_val_fail.stance_type_ids.clone(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValShipStanceItemInfo {
    item_id: rc::SolItemId,
    type_id: rc::EItemId,
}
impl From<&rc::SolValShipStanceItemInfo> for HValShipStanceItemInfo {
    fn from(core_item_info: &rc::SolValShipStanceItemInfo) -> Self {
        Self {
            item_id: core_item_info.item_id,
            type_id: core_item_info.type_id,
        }
    }
}
