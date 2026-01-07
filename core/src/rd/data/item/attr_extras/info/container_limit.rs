use itertools::Itertools;

use crate::{
    ad::AItemGrpId,
    misc::Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct RItemContLimit {
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(in crate::rd::data::item::attr_extras) fn get_item_container_limit(
    item_attrs: &RMap<RAttrId, Value>,
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
    .filter_map(|attr_rid| attr_rid.and_then(|attr_rid| item_attrs.get(&attr_rid)))
    .map(|v| AItemGrpId::from_f64_rounded(v.into_f64()))
    .unique()
    .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RItemContLimit { group_ids })
}
