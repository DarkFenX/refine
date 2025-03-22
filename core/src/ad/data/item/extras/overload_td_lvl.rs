use crate::{
    ac,
    ad::{AAttrId, AAttrVal, ASkillLevel},
    util::StMap,
};

pub(super) fn get_overload_td_lvl(item_attrs: &StMap<AAttrId, AAttrVal>) -> Option<ASkillLevel> {
    item_attrs
        .get(&ac::attrs::REQUIRED_THERMODYNAMICS_SKILL)
        .map(|v| v.round() as ASkillLevel)
}
