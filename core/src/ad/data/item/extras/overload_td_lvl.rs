use crate::{
    ac,
    ad::{AAttrId, AAttrVal, ASkillLevel},
    util::HMap,
};

pub(super) fn get_overload_td_lvl(item_attrs: &HMap<AAttrId, AAttrVal>) -> Option<ASkillLevel> {
    item_attrs
        .get(&ac::attrs::REQUIRED_THERMODYNAMICS_SKILL)
        .map(|v| v.round() as ASkillLevel)
}
