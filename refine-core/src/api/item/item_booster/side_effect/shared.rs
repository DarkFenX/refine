use crate::{
    ad::AAttrId,
    rd::{RData, REffectId},
};

pub(super) fn get_se_chance_attr_aid_by_effect_rid(r_data: &RData, effect_rid: REffectId) -> Option<AAttrId> {
    let attr_rid = r_data.get_effect_by_rid(effect_rid).chance_attr_rid?;
    Some(r_data.get_attr_by_rid(attr_rid).aid)
}
