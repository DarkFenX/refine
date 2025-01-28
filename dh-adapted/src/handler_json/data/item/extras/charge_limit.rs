#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItemChargeLimit {
    pub(in crate::handler_json) group_ids: Vec<rc::EItemGrpId>,
}
impl From<&rc::ad::AItemChargeLimit> for CItemChargeLimit {
    fn from(a_item_ship_limit: &rc::ad::AItemChargeLimit) -> Self {
        CItemChargeLimit {
            group_ids: a_item_ship_limit.group_ids.clone(),
        }
    }
}
impl Into<rc::ad::AItemChargeLimit> for &CItemChargeLimit {
    fn into(self) -> rc::ad::AItemChargeLimit {
        rc::ad::AItemChargeLimit {
            group_ids: self.group_ids.clone(),
        }
    }
}
