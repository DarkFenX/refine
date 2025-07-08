use super::shared::get_range;
use crate::{
    ac, ad,
    def::{AttrVal, ItemKey, OF},
    svc::{SvcCtx, calc::Calc},
    uad::UadProjRange,
};

pub(crate) fn get_proj_attrs_aoe_burst(a_effect: &ad::AEffect) -> [Option<ad::AAttrId>; 2] {
    [a_effect.range_attr_id, Some(ac::attrs::DOOMSDAY_AOE_RANGE)]
}

pub(crate) fn get_proj_mult_aoe_burst(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: ItemKey,
    a_effect: &ad::AEffect,
    prange: UadProjRange,
) -> AttrVal {
    // Doomsday projectiles are launched from center of the ship, and range is extended by aoe range
    let affector_optimal = get_range(ctx, calc, affector_key, a_effect.range_attr_id);
    let affector_aoe = get_range(ctx, calc, affector_key, Some(ac::attrs::DOOMSDAY_AOE_RANGE));
    match prange.get_c2s() <= affector_optimal + affector_aoe {
        true => OF(1.0),
        false => OF(0.0),
    }
}
