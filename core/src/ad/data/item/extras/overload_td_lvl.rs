use crate::{
    ac,
    ad::{AAttrId, AAttrVal, ASkillLevel, ASkillLevelInner},
    util::RMap,
};

pub(super) fn get_overload_td_lvl(item_attrs: &RMap<AAttrId, AAttrVal>) -> Option<ASkillLevel> {
    item_attrs
        .get(&ac::attrs::REQUIRED_THERMODYNAMICS_SKILL)
        .map(|v| ASkillLevel::new(v.round() as ASkillLevelInner))
}
