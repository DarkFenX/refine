use itertools::Itertools;

use crate::{ac, ad, util::RMap};

const GROUP_ATTRS: [ad::AAttrId; 2] = [ac::attrs::ALLOWED_DRONE_GROUP1, ac::attrs::ALLOWED_DRONE_GROUP2];

#[derive(Clone)]
pub(crate) struct RShipDroneLimit {
    pub(crate) group_ids: Vec<ad::AItemGrpId>,
}

pub(super) fn get_ship_drone_limit(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<RShipDroneLimit> {
    let group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as ad::AItemGrpId)
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RShipDroneLimit { group_ids })
}
