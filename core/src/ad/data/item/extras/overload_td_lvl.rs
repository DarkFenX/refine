use crate::{
    defs::{AttrVal, EAttrId, SkillLevel},
    ec,
    util::StMap,
};

pub(super) fn get_overload_td_lvl(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<SkillLevel> {
    item_attrs
        .get(&ec::attrs::REQUIRED_THERMODYNAMICS_SKILL)
        .map(|v| v.round() as SkillLevel)
}
