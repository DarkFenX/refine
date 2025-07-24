use crate::{ac, ad, util::RMap};

pub(super) fn get_overload_td_lvl(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<ad::ASkillLevel> {
    item_attrs
        .get(&ac::attrs::REQUIRED_THERMODYNAMICS_SKILL)
        .map(|v| ad::ASkillLevel::new(v.round() as i32))
}
