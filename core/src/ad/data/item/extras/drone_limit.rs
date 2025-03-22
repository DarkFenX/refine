use itertools::Itertools;

use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AItemGrpId},
    util::StMap,
};

static GROUP_ATTRS: [AAttrId; 2] = [ac::attrs::ALLOWED_DRONE_GROUP1, ac::attrs::ALLOWED_DRONE_GROUP2];

/// If a ship is limited, it can only use drones from specified groups.
#[derive(Clone)]
pub struct AShipDroneLimit {
    pub group_ids: Vec<AItemGrpId>,
}

pub(super) fn get_ship_drone_limit(item_attrs: &StMap<AAttrId, AAttrVal>) -> Option<AShipDroneLimit> {
    let group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as AItemGrpId)
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(AShipDroneLimit { group_ids })
}
