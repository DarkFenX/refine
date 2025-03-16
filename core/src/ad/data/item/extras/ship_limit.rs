use itertools::Itertools;

use crate::{
    defs::{AttrVal, EAttrId, EItemGrpId, EItemId},
    ec,
    util::StMap,
};

static TYPE_ATTRS: [EAttrId; 12] = [
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
    ec::attrs::FITS_TO_SHIP_TYPE,
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

pub(super) fn get_item_ship_limit(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<AItemShipLimit> {
    let limit_type_ids = TYPE_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as EItemId)
        .unique()
        .collect_vec();
    let limit_group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as EItemGrpId)
        .unique()
        .collect_vec();
    if limit_type_ids.is_empty() && limit_group_ids.is_empty() {
        return None;
    }
    Some(AItemShipLimit {
        type_ids: limit_type_ids,
        group_ids: limit_group_ids,
    })
}
