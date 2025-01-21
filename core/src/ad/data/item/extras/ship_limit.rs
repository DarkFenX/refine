use itertools::Itertools;

use crate::{
    defs::{AttrVal, EAttrId, EItemGrpId, EItemId},
    ec,
    util::StMap,
};

static TYPE_ATTRS: [EAttrId; 11] = [
    ec::attrs::CAN_FIT_SHIP_TYPE1,
    ec::attrs::CAN_FIT_SHIP_TYPE2,
    ec::attrs::CAN_FIT_SHIP_TYPE3,
    ec::attrs::CAN_FIT_SHIP_TYPE4,
    ec::attrs::CAN_FIT_SHIP_TYPE5,
    ec::attrs::CAN_FIT_SHIP_TYPE6,
    ec::attrs::CAN_FIT_SHIP_TYPE7,
    ec::attrs::CAN_FIT_SHIP_TYPE8,
    ec::attrs::CAN_FIT_SHIP_TYPE9,
    ec::attrs::CAN_FIT_SHIP_TYPE10,
    ec::attrs::CAN_FIT_SHIP_TYPE11,
];

static GROUP_ATTRS: [EAttrId; 20] = [
    ec::attrs::CAN_FIT_SHIP_GROUP1,
    ec::attrs::CAN_FIT_SHIP_GROUP2,
    ec::attrs::CAN_FIT_SHIP_GROUP3,
    ec::attrs::CAN_FIT_SHIP_GROUP4,
    ec::attrs::CAN_FIT_SHIP_GROUP5,
    ec::attrs::CAN_FIT_SHIP_GROUP6,
    ec::attrs::CAN_FIT_SHIP_GROUP7,
    ec::attrs::CAN_FIT_SHIP_GROUP8,
    ec::attrs::CAN_FIT_SHIP_GROUP9,
    ec::attrs::CAN_FIT_SHIP_GROUP10,
    ec::attrs::CAN_FIT_SHIP_GROUP11,
    ec::attrs::CAN_FIT_SHIP_GROUP12,
    ec::attrs::CAN_FIT_SHIP_GROUP13,
    ec::attrs::CAN_FIT_SHIP_GROUP14,
    ec::attrs::CAN_FIT_SHIP_GROUP15,
    ec::attrs::CAN_FIT_SHIP_GROUP16,
    ec::attrs::CAN_FIT_SHIP_GROUP17,
    ec::attrs::CAN_FIT_SHIP_GROUP18,
    ec::attrs::CAN_FIT_SHIP_GROUP19,
    ec::attrs::CAN_FIT_SHIP_GROUP20,
];

/// If a module is limited, it can only be fit to a ship of specific type or group.
#[derive(Clone)]
pub struct AItemShipLimit {
    pub type_ids: Vec<EItemId>,
    pub group_ids: Vec<EItemGrpId>,
}
impl AItemShipLimit {
    pub(crate) fn new(type_ids: Vec<EItemId>, group_ids: Vec<EItemGrpId>) -> Self {
        Self { type_ids, group_ids }
    }
}

pub(super) fn get_item_ship_limit(attrs: &StMap<EAttrId, AttrVal>) -> Option<AItemShipLimit> {
    let type_ids = TYPE_ATTRS
        .iter()
        .filter_map(|a| attrs.get(a))
        .map(|v| v.round() as EItemId)
        .unique()
        .collect_vec();
    let group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| attrs.get(a))
        .map(|v| v.round() as EItemGrpId)
        .unique()
        .collect_vec();
    if type_ids.is_empty() && group_ids.is_empty() {
        return None;
    }
    Some(AItemShipLimit::new(type_ids, group_ids))
}
