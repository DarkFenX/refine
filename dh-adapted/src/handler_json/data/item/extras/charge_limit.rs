#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItemChargeLimit {
    pub(in crate::handler_json) group_ids: Vec<rc::EItemGrpId>,
}
impl From<&rc::ad::AItemChargeLimit> for CItemChargeLimit {
    fn from(a_item_charge_limit: &rc::ad::AItemChargeLimit) -> Self {
        Self {
            group_ids: a_item_charge_limit.group_ids.clone(),
        }
    }
}
impl From<&CItemChargeLimit> for rc::ad::AItemChargeLimit {
    fn from(c_item_charge_limit: &CItemChargeLimit) -> Self {
        Self {
            group_ids: c_item_charge_limit.group_ids.clone(),
        }
    }
}
