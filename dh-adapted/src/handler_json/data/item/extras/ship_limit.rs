#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItemShipLimit {
    pub(in crate::handler_json) type_ids: Vec<rc::EItemId>,
    pub(in crate::handler_json) group_ids: Vec<rc::EItemGrpId>,
}
impl From<&rc::ad::AItemShipLimit> for CItemShipLimit {
    fn from(a_item_ship_limit: &rc::ad::AItemShipLimit) -> Self {
        CItemShipLimit {
            type_ids: a_item_ship_limit.type_ids.clone(),
            group_ids: a_item_ship_limit.group_ids.clone(),
        }
    }
}
impl Into<rc::ad::AItemShipLimit> for &CItemShipLimit {
    fn into(self) -> rc::ad::AItemShipLimit {
        rc::ad::AItemShipLimit {
            type_ids: self.type_ids.clone(),
            group_ids: self.group_ids.clone(),
        }
    }
}
