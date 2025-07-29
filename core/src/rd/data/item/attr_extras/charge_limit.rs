use itertools::Itertools;

use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AItemGrpId},
    util::RMap,
};

const GROUP_ATTRS: [AAttrId; 5] = [
    ac::attrs::CHARGE_GROUP1,
    ac::attrs::CHARGE_GROUP2,
    ac::attrs::CHARGE_GROUP3,
    ac::attrs::CHARGE_GROUP4,
    ac::attrs::CHARGE_GROUP5,
];

#[derive(Clone)]
pub(crate) struct RItemChargeLimit {
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(super) fn get_item_charge_limit(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<RItemChargeLimit> {
    let group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as AItemGrpId)
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RItemChargeLimit { group_ids })
}
