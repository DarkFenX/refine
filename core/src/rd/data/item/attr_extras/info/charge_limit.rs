use itertools::Itertools;

use crate::{
    ad::AItemGrpId,
    misc::Value,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

#[derive(Clone)]
pub(crate) struct RItemChargeLimit {
    pub(crate) group_ids: Vec<AItemGrpId>,
}

pub(in crate::rd::data::item::attr_extras) fn get_item_charge_limit(
    item_attrs: &RMap<RAttrId, Value>,
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
    .map(|v| AItemGrpId::from_f64_rounded(v.into_f64()))
    .unique()
    .collect_vec();
    if group_ids.is_empty() {
        return None;
    }
    Some(RItemChargeLimit { group_ids })
}
