use crate::{
    ac,
    ad::{AAttrId, AEffect},
};

pub(in crate::nd::effect::data) fn get_simple_mod_proj_attrs(a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [a_effect.range_attr_id, None]
}

pub(in crate::nd::effect::data) fn get_full_mod_proj_attrs(a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [a_effect.range_attr_id, a_effect.falloff_attr_id]
}

pub(in crate::nd::effect::data) fn get_aoe_dd_mod_proj_attrs(_a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [Some(ac::attrs::MAX_RANGE), None]
}

pub(in crate::nd::effect::data) fn get_aoe_burst_mod_proj_attrs(_a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [Some(ac::attrs::MAX_RANGE), Some(ac::attrs::DOOMSDAY_AOE_RANGE)]
}
