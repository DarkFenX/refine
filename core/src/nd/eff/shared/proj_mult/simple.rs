use super::shared::get_range;
use crate::{
    ad,
    def::{AttrVal, OF},
    rd,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjRange},
};

pub(crate) fn get_proj_attrs_simple(a_effect: &ad::AEffect) -> [Option<ad::AAttrId>; 2] {
    [a_effect.range_attr_id, None]
}

pub(crate) fn get_proj_mult_simple_s2s(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UItemKey,
    r_effect: &rd::REffect,
    prange: UProjRange,
) -> AttrVal {
    let affector_optimal = get_range(ctx, calc, affector_key, r_effect.get_range_attr_id());
    match prange.get_s2s() <= affector_optimal {
        true => OF(1.0),
        false => OF(0.0),
    }
}
