use ordered_float::Float;

use super::shared::get_range;
use crate::{
    ad,
    def::{AttrVal, OF},
    rd,
    svc::{SvcCtx, calc::Calc},
    uad::{UadItemKey, UadProjRange},
};

pub(crate) fn get_proj_attrs_normal(a_effect: &ad::AEffect) -> [Option<ad::AAttrId>; 2] {
    [a_effect.range_attr_id, a_effect.falloff_attr_id]
}

pub(crate) fn get_proj_mult_normal_restricted_s2s(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UadItemKey,
    r_effect: &rd::REffect,
    prange: UadProjRange,
) -> AttrVal {
    get_proj_mult_normal(ctx, calc, affector_key, r_effect, prange.get_s2s(), true)
}

pub(crate) fn get_proj_mult_normal_unrestricted_s2s(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UadItemKey,
    r_effect: &rd::REffect,
    prange: UadProjRange,
) -> AttrVal {
    get_proj_mult_normal(ctx, calc, affector_key, r_effect, prange.get_s2s(), false)
}

fn get_proj_mult_normal(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UadItemKey,
    r_effect: &rd::REffect,
    prange: AttrVal,
    restricted: bool,
) -> AttrVal {
    let affector_optimal = get_range(ctx, calc, affector_key, r_effect.get_range_attr_id());
    let affector_falloff = get_range(ctx, calc, affector_key, r_effect.get_falloff_attr_id());
    // Calculate actual range multiplier after collecting all the data
    match affector_falloff > OF(0.0) {
        true => match restricted && prange > affector_optimal + OF(3.0) * affector_falloff {
            true => OF(0.0),
            false => Float::powf(
                OF(0.5),
                (Float::max(OF(0.0), prange - affector_optimal) / affector_falloff).powi(2),
            ),
        },
        false => match prange <= affector_optimal {
            true => OF(1.0),
            false => OF(0.0),
        },
    }
}
