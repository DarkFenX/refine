use itertools::Itertools;

use crate::{
    ad::{AAttrVal, AItemGrpId},
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct RItemChargeLimit {
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(in crate::rd::data::item::attr_extras) fn get_item_charge_limit(
    item_attrs: &RMap<RAttrId, AAttrVal>,
    attr_consts: &RAttrConsts,
) -> Option<RItemChargeLimit> {
    let group_ids = [
        attr_consts.charge_group1,
        attr_consts.charge_group2,
        attr_consts.charge_group3,
        attr_consts.charge_group4,
        attr_consts.charge_group5,
    ]
    .iter()
    .filter_map(|opt| opt.and_then(|attr_key| item_attrs.get(&attr_key)))
    .map(|v| v.round() as AItemGrpId)
    .unique()
    .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RItemChargeLimit { group_ids })
}
