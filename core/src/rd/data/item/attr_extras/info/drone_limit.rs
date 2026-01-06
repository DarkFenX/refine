use itertools::Itertools;

use crate::{
    ad::AItemGrpId,
    misc::Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct RShipDroneLimit {
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(in crate::rd::data::item::attr_extras) fn get_ship_drone_limit(
    item_attrs: &RMap<RAttrId, Value>,
    attr_consts: &RAttrConsts,
) -> Option<RShipDroneLimit> {
    let group_ids = [attr_consts.allowed_drone_group1, attr_consts.allowed_drone_group2]
        .iter()
        .filter_map(|attr_rid| attr_rid.and_then(|attr_rid| item_attrs.get(&attr_rid)))
        .map(|v| AItemGrpId::from_f64_rounded(v.into()))
        .unique()
        .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RShipDroneLimit { group_ids })
}
