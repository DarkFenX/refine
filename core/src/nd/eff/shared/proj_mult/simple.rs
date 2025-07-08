use super::shared::get_range;
use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    svc::{SvcCtx, calc::Calc},
    uad::UadProjRange,
};

pub(crate) fn get_proj_attrs_simple(a_effect: &ad::AEffect) -> [Option<ad::AAttrId>; 2] {
    [a_effect.range_attr_id, None]
}

pub(crate) fn get_proj_mult_simple_s2s(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: ItemKey,
    a_effect: &ad::AEffect,
    prange: UadProjRange,
) -> AttrVal {
    let affector_optimal = get_range(ctx, calc, affector_key, a_effect.range_attr_id);
    match prange.get_s2s() <= affector_optimal {
        true => OF(1.0),
        false => OF(0.0),
    }
}
