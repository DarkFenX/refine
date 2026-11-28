use itertools::Itertools;

use crate::{
    Value,
    ad::AItemGrpId,
    rd::{RAttrConsts, RAttrId},
    util::RMap,
};

#[derive(Clone, Default)]
pub(crate) struct RItemChargeLimit {
    pub(crate) group_ids: Box<[AItemGrpId]>,
}
impl RItemChargeLimit {
    pub(in crate::rd::data::item::base) fn build(item_attrs: &RMap<RAttrId, Value>, attr_consts: &RAttrConsts) -> Self {
        let group_ids = [
            attr_consts.charge_group1,
            attr_consts.charge_group2,
            attr_consts.charge_group3,
            attr_consts.charge_group4,
            attr_consts.charge_group5,
        ]
        .iter()
        .filter_map(|opt| {
            opt.and_then(|attr_key| item_attrs.get(&attr_key))
                .and_then(|v| AItemGrpId::try_from_f64_rounded(v.into_f64()))
        })
        .unique()
        .collect();
        Self { group_ids }
    }
    pub(crate) fn accepts_charge(&self, charge_grp_aid: AItemGrpId) -> bool {
        self.group_ids.contains(&charge_grp_aid)
    }
}
