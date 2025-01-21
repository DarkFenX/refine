use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HShipLimitValFail {
    ship_type_id: Option<rc::EItemId>,
    ship_group_id: Option<rc::EItemGrpId>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    mismatches: HashMap<rc::SolItemId, HShipLimitAllowed>,
}
impl From<&rc::SolShipLimitValFail> for HShipLimitValFail {
    fn from(core_val_fail: &rc::SolShipLimitValFail) -> Self {
        Self {
            ship_type_id: core_val_fail.ship_type_id,
            ship_group_id: core_val_fail.ship_group_id,
            mismatches: core_val_fail.mismatches.iter().map(|v| (v.item_id, v.into())).collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HShipLimitAllowed {
    allowed_type_ids: Vec<rc::EItemId>,
    allowed_group_ids: Vec<rc::EItemGrpId>,
}
impl From<&rc::SolShipLimitMismatch> for HShipLimitAllowed {
    fn from(core_mismatch: &rc::SolShipLimitMismatch) -> Self {
        Self {
            allowed_type_ids: core_mismatch.allowed_type_ids.clone(),
            allowed_group_ids: core_mismatch.allowed_group_ids.clone(),
        }
    }
}
