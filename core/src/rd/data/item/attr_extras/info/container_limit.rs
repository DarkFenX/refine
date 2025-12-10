use itertools::Itertools;

use crate::{
    ad::{AAttrVal, AItemGrpId},
    rd::{RAttrConsts, RAttrKey},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct RItemContLimit {
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(in crate::rd::data::item::attr_extras) fn get_item_container_limit(
    item_attrs: &RMap<RAttrKey, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<RItemContLimit> {
    let group_ids = [
        attr_consts.launcher_group,
        attr_consts.launcher_group2,
        attr_consts.launcher_group3,
        attr_consts.launcher_group4,
        attr_consts.launcher_group5,
        attr_consts.launcher_group6,
    ]
    .iter()
    .filter_map(|opt| opt.and_then(|attr_key| item_attrs.get(&attr_key)))
    .map(|v| v.round() as AItemGrpId)
    .unique()
    .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RItemContLimit { group_ids })
}
