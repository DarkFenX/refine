use itertools::Itertools;

use crate::{
    ad::{AAttrVal, AItemGrpId},
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct RShipDroneLimit {
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(in crate::rd::data::item::attr_extras) fn get_ship_drone_limit(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<RShipDroneLimit> {
    let group_ids = [attr_consts.allowed_drone_group1, attr_consts.allowed_drone_group2]
        .iter()
        .filter_map(|opt| opt.and_then(|attr_key| item_attrs.get(&attr_key)))
        .map(|v| v.round() as AItemGrpId)
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RShipDroneLimit { group_ids })
}
