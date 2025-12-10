use itertools::Itertools;

use crate::{
    ac,
    ad::{AAttrVal, AItemGrpId, AItemId},
    rd::{RAttrConsts, RAttrKey},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct RItemShipLimit {
    pub(crate) type_ids: Vec<AItemId>,
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(in crate::rd::data::item::attr_extras) fn get_item_ship_limit(
    a_item_id: AItemId,
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<RItemShipLimit> {
    let mut limit_type_ids = [
        attr_consts.can_fit_ship_type1,
        attr_consts.can_fit_ship_type2,
        attr_consts.can_fit_ship_type3,
        attr_consts.can_fit_ship_type4,
        attr_consts.can_fit_ship_type5,
        attr_consts.can_fit_ship_type6,
        attr_consts.can_fit_ship_type7,
        attr_consts.can_fit_ship_type8,
        attr_consts.can_fit_ship_type9,
        attr_consts.can_fit_ship_type10,
        attr_consts.can_fit_ship_type11,
        attr_consts.can_fit_ship_type12,
        attr_consts.fits_to_ship_type,
    ]
    .iter()
    .filter_map(|opt| opt.and_then(|attr_key| item_attrs.get(&attr_key)))
    .map(|v| v.round() as AItemId)
    .unique()
    .collect_vec();
    let limit_group_ids = [
        attr_consts.can_fit_ship_group1,
        attr_consts.can_fit_ship_group2,
        attr_consts.can_fit_ship_group3,
        attr_consts.can_fit_ship_group4,
        attr_consts.can_fit_ship_group5,
        attr_consts.can_fit_ship_group6,
        attr_consts.can_fit_ship_group7,
        attr_consts.can_fit_ship_group8,
        attr_consts.can_fit_ship_group9,
        attr_consts.can_fit_ship_group10,
        attr_consts.can_fit_ship_group11,
        attr_consts.can_fit_ship_group12,
        attr_consts.can_fit_ship_group13,
        attr_consts.can_fit_ship_group14,
        attr_consts.can_fit_ship_group15,
        attr_consts.can_fit_ship_group16,
        attr_consts.can_fit_ship_group17,
        attr_consts.can_fit_ship_group18,
        attr_consts.can_fit_ship_group19,
        attr_consts.can_fit_ship_group20,
    ]
    .iter()
    .filter_map(|opt| opt.and_then(|attr_key| item_attrs.get(&attr_key)))
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
    Some(RItemShipLimit {
        type_ids: limit_type_ids,
        group_ids: limit_group_ids,
    })
}
