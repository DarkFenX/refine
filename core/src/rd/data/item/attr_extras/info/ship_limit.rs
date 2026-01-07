use itertools::Itertools;

use crate::{
    ad::{AItemGrpId, AItemId},
    misc::Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct RItemShipLimit {
    pub(crate) type_ids: Vec<AItemId>,
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(in crate::rd::data::item::attr_extras) fn get_item_ship_limit(
    item_aid: AItemId,
    item_attrs: &RMap<RAttrId, Value>,
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
    .filter_map(|attr_rid| attr_rid.and_then(|attr_rid| item_attrs.get(&attr_rid)))
    .map(|v| AItemId::from_f64_rounded(v.into()))
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
    .filter_map(|attr_rid| attr_rid.and_then(|attr_rid| item_attrs.get(&attr_rid)))
    .map(|v| AItemGrpId::from_f64_rounded(v.into()))
    .unique()
    .collect_vec();
    match item_aid {
        AItemId::CONFESSOR_DEFENSE_MODE => limit_type_ids.push(AItemId::CONFESSOR),
        AItemId::CONFESSOR_PROPULSION_MODE => limit_type_ids.push(AItemId::CONFESSOR),
        AItemId::CONFESSOR_SHARPSHOOTER_MODE => limit_type_ids.push(AItemId::CONFESSOR),
        AItemId::HECATE_DEFENSE_MODE => limit_type_ids.push(AItemId::HECATE),
        AItemId::HECATE_PROPULSION_MODE => limit_type_ids.push(AItemId::HECATE),
        AItemId::HECATE_SHARPSHOOTER_MODE => limit_type_ids.push(AItemId::HECATE),
        AItemId::JACKDAW_DEFENSE_MODE => limit_type_ids.push(AItemId::JACKDAW),
        AItemId::JACKDAW_PROPULSION_MODE => limit_type_ids.push(AItemId::JACKDAW),
        AItemId::JACKDAW_SHARPSHOOTER_MODE => limit_type_ids.push(AItemId::JACKDAW),
        AItemId::SVIPUL_DEFENSE_MODE => limit_type_ids.push(AItemId::SVIPUL),
        AItemId::SVIPUL_PROPULSION_MODE => limit_type_ids.push(AItemId::SVIPUL),
        AItemId::SVIPUL_SHARPSHOOTER_MODE => limit_type_ids.push(AItemId::SVIPUL),
        AItemId::SKUA_DEFENSE_MODE => limit_type_ids.push(AItemId::SKUA),
        AItemId::SKUA_PROPULSION_MODE => limit_type_ids.push(AItemId::SKUA),
        AItemId::SKUA_SHARPSHOOTER_MODE => limit_type_ids.push(AItemId::SKUA),
        AItemId::ANHINGA_PRIMARY_MODE => limit_type_ids.push(AItemId::ANHINGA),
        AItemId::ANHINGA_SECONDARY_MODE => limit_type_ids.push(AItemId::ANHINGA),
        AItemId::ANHINGA_TERTIARY_MODE => limit_type_ids.push(AItemId::ANHINGA),
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
