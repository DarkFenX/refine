use itertools::Itertools;

use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AItemGrpId},
    util::RMap,
};

const GROUP_ATTRS: [AAttrId; 6] = [
    ac::attrs::LAUNCHER_GROUP,
    ac::attrs::LAUNCHER_GROUP2,
    ac::attrs::LAUNCHER_GROUP3,
    ac::attrs::LAUNCHER_GROUP4,
    ac::attrs::LAUNCHER_GROUP5,
    ac::attrs::LAUNCHER_GROUP6,
];

#[derive(Clone)]
pub(crate) struct AItemContLimit {
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(super) fn get_item_container_limit(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<AItemContLimit> {
    let group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as AItemGrpId)
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(AItemContLimit { group_ids })
}
