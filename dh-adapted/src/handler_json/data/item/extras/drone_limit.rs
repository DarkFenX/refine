#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CShipDroneLimit {
    pub(in crate::handler_json) group_ids: Vec<rc::EItemGrpId>,
}
impl From<&rc::ad::AShipDroneLimit> for CShipDroneLimit {
    fn from(a_ship_drone_limit: &rc::ad::AShipDroneLimit) -> Self {
        Self {
            group_ids: a_ship_drone_limit.group_ids.clone(),
        }
    }
}
impl From<&CShipDroneLimit> for rc::ad::AShipDroneLimit {
    fn from(c_ship_drone_limit: &CShipDroneLimit) -> Self {
        Self {
            group_ids: c_ship_drone_limit.group_ids.clone(),
        }
    }
}
