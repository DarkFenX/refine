use itertools::Itertools;

use crate::{ac, ad, util::RMap};

const GROUP_ATTRS: [ad::AAttrId; 5] = [
    ac::attrs::CHARGE_GROUP1,
    ac::attrs::CHARGE_GROUP2,
    ac::attrs::CHARGE_GROUP3,
    ac::attrs::CHARGE_GROUP4,
    ac::attrs::CHARGE_GROUP5,
];

#[derive(Clone)]
pub(crate) struct RItemChargeLimit {
    pub(crate) group_ids: Vec<ad::AItemGrpId>,
}

pub(super) fn get_item_charge_limit(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<RItemChargeLimit> {
    let group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as ad::AItemGrpId)
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RItemChargeLimit { group_ids })
}
