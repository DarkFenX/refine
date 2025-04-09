use itertools::Itertools;

use crate::{
    ac,
    ad::{AAttrId, AAttrVal, AItemGrpId, AItemId},
    util::RMap,
};

const TYPE_ATTRS: [AAttrId; 12] = [
    ac::attrs::CAN_FIT_SHIP_TYPE1,
    ac::attrs::CAN_FIT_SHIP_TYPE2,
    ac::attrs::CAN_FIT_SHIP_TYPE3,
    ac::attrs::CAN_FIT_SHIP_TYPE4,
    ac::attrs::CAN_FIT_SHIP_TYPE5,
    ac::attrs::CAN_FIT_SHIP_TYPE6,
    ac::attrs::CAN_FIT_SHIP_TYPE7,
    ac::attrs::CAN_FIT_SHIP_TYPE8,
    ac::attrs::CAN_FIT_SHIP_TYPE9,
    ac::attrs::CAN_FIT_SHIP_TYPE10,
    ac::attrs::CAN_FIT_SHIP_TYPE11,
    ac::attrs::FITS_TO_SHIP_TYPE,
];

const GROUP_ATTRS: [AAttrId; 20] = [
    ac::attrs::CAN_FIT_SHIP_GROUP1,
    ac::attrs::CAN_FIT_SHIP_GROUP2,
    ac::attrs::CAN_FIT_SHIP_GROUP3,
    ac::attrs::CAN_FIT_SHIP_GROUP4,
    ac::attrs::CAN_FIT_SHIP_GROUP5,
    ac::attrs::CAN_FIT_SHIP_GROUP6,
    ac::attrs::CAN_FIT_SHIP_GROUP7,
    ac::attrs::CAN_FIT_SHIP_GROUP8,
    ac::attrs::CAN_FIT_SHIP_GROUP9,
    ac::attrs::CAN_FIT_SHIP_GROUP10,
    ac::attrs::CAN_FIT_SHIP_GROUP11,
    ac::attrs::CAN_FIT_SHIP_GROUP12,
    ac::attrs::CAN_FIT_SHIP_GROUP13,
    ac::attrs::CAN_FIT_SHIP_GROUP14,
    ac::attrs::CAN_FIT_SHIP_GROUP15,
    ac::attrs::CAN_FIT_SHIP_GROUP16,
    ac::attrs::CAN_FIT_SHIP_GROUP17,
    ac::attrs::CAN_FIT_SHIP_GROUP18,
    ac::attrs::CAN_FIT_SHIP_GROUP19,
    ac::attrs::CAN_FIT_SHIP_GROUP20,
];

/// If a module is limited, it can only be fit to a ship of specific type or group.
#[derive(Clone)]
pub struct AItemShipLimit {
    pub type_ids: Vec<AItemId>,
    pub group_ids: Vec<AItemGrpId>,
}

pub(super) fn get_item_ship_limit(a_item_id: AItemId, item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<AItemShipLimit> {
    let mut limit_type_ids = TYPE_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as AItemId)
        .unique()
        .collect_vec();
    let limit_group_ids = GROUP_ATTRS
        .iter()
        .filter_map(|a| item_attrs.get(a))
        .map(|v| v.round() as AItemGrpId)
        .unique()
        .collect_vec();
    match a_item_id {
        ac::items::CONFESSOR_DEFENSE_MODE => limit_type_ids.push(ac::items::CONFESSOR),
        ac::items::CONFESSOR_PROPULSION_MODE => limit_type_ids.push(ac::items::CONFESSOR),
        ac::items::CONFESSOR_SHARPSHOOTER_MODE => limit_type_ids.push(ac::items::CONFESSOR),
        ac::items::HECATE_DEFENSE_MODE => limit_type_ids.push(ac::items::HECATE),
        ac::items::HECATE_PROPULSION_MODE => limit_type_ids.push(ac::items::HECATE),
        ac::items::HECATE_SHARPSHOOTER_MODE => limit_type_ids.push(ac::items::HECATE),
        ac::items::JACKDAW_DEFENSE_MODE => limit_type_ids.push(ac::items::JACKDAW),
        ac::items::JACKDAW_PROPULSION_MODE => limit_type_ids.push(ac::items::JACKDAW),
        ac::items::JACKDAW_SHARPSHOOTER_MODE => limit_type_ids.push(ac::items::JACKDAW),
        ac::items::SVIPUL_DEFENSE_MODE => limit_type_ids.push(ac::items::SVIPUL),
        ac::items::SVIPUL_PROPULSION_MODE => limit_type_ids.push(ac::items::SVIPUL),
        ac::items::SVIPUL_SHARPSHOOTER_MODE => limit_type_ids.push(ac::items::SVIPUL),
        _ => (),
    }
    if limit_type_ids.is_empty() && limit_group_ids.is_empty() {
        return None;
    }
    Some(AItemShipLimit {
        type_ids: limit_type_ids,
        group_ids: limit_group_ids,
    })
}
