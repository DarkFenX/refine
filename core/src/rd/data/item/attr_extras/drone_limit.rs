use itertools::Itertools;

use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AItemGrpId},
    util::RMap,
};

const GROUP_ATTRS: [AAttrId; 2] = [ac::attrs::ALLOWED_DRONE_GROUP1, ac::attrs::ALLOWED_DRONE_GROUP2];

#[derive(Clone)]
pub(crate) struct RShipDroneLimit {
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(super) fn get_ship_drone_limit(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<RShipDroneLimit> {
    let group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as AItemGrpId)
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RShipDroneLimit { group_ids })
}
