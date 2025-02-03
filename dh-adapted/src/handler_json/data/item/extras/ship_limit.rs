#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItemShipLimit {
    pub(in crate::handler_json) type_ids: Vec<rc::EItemId>,
    pub(in crate::handler_json) group_ids: Vec<rc::EItemGrpId>,
}
impl From<&rc::ad::AItemShipLimit> for CItemShipLimit {
    fn from(a_item_ship_limit: &rc::ad::AItemShipLimit) -> Self {
        Self {
            type_ids: a_item_ship_limit.type_ids.clone(),
            group_ids: a_item_ship_limit.group_ids.clone(),
        }
    }
}
impl From<&CItemShipLimit> for rc::ad::AItemShipLimit {
    fn from(c_item_ship_limit: &CItemShipLimit) -> Self {
        Self {
            type_ids: c_item_ship_limit.type_ids.clone(),
            group_ids: c_item_ship_limit.group_ids.clone(),
        }
    }
}
