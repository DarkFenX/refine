use itertools::Itertools;

use crate::{
    defs::{AttrVal, EAttrId, EItemGrpId},
    ec,
    util::StMap,
};

static GROUP_ATTRS: [EAttrId; 5] = [
    ec::attrs::CHARGE_GROUP1,
    ec::attrs::CHARGE_GROUP2,
    ec::attrs::CHARGE_GROUP3,
    ec::attrs::CHARGE_GROUP4,
    ec::attrs::CHARGE_GROUP5,
];

/// If a module is limited, it can only load charges of specific group.
#[derive(Clone)]
pub struct AItemChargeLimit {
    pub group_ids: Vec<EItemGrpId>,
}
impl AItemChargeLimit {
    pub(crate) fn new(group_ids: Vec<EItemGrpId>) -> Self {
        Self { group_ids }
    }
}

pub(super) fn get_item_charge_limit(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<AItemChargeLimit> {
    let group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as EItemGrpId)
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(AItemChargeLimit::new(group_ids))
}
