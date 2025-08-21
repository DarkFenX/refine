use crate::{
    ac,
    ad::{AAttrId, AEffect},
};

pub(in crate::nd::eff) fn get_simple_mod_proj_attrs(a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [a_effect.range_attr_id, None]
}

pub(in crate::nd::eff) fn get_full_mod_proj_attrs(a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [a_effect.range_attr_id, a_effect.falloff_attr_id]
}

pub(in crate::nd::eff) fn get_aoe_burst_mod_proj_attrs(a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [a_effect.range_attr_id, Some(ac::attrs::DOOMSDAY_AOE_RANGE)]
}
