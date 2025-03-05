use itertools::Itertools;

use crate::{
    defs::{AttrVal, EAttrId, EItemGrpId},
    ec,
    util::StMap,
};

static GROUP_ATTRS: [EAttrId; 2] = [ec::attrs::ALLOWED_DRONE_GROUP1, ec::attrs::ALLOWED_DRONE_GROUP2];

/// If a ship is limited, it can only use drones from specified groups.
#[derive(Clone)]
pub struct AShipDroneLimit {
    pub group_ids: Vec<EItemGrpId>,
}
impl AShipDroneLimit {
    pub(crate) fn new(group_ids: Vec<EItemGrpId>) -> Self {
        Self { group_ids }
    }
}

pub(super) fn get_ship_drone_limit(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<AShipDroneLimit> {
    let group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as EItemGrpId)
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(AShipDroneLimit::new(group_ids))
}
