use ordered_float::Float;

use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    svc::{SvcCtx, calc::Calc},
    uad::UadProjRange,
};

pub(crate) fn get_proj_mult_normal_restricted_s2s(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: ItemKey,
    a_effect: &ad::AEffect,
    prange: UadProjRange,
) -> AttrVal {
    get_proj_mult_normal(ctx, calc, affector_key, a_effect, prange.s2s, true)
}

pub(crate) fn get_proj_mult_normal_unrestricted_s2s(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: ItemKey,
    a_effect: &ad::AEffect,
    prange: UadProjRange,
) -> AttrVal {
    get_proj_mult_normal(ctx, calc, affector_key, a_effect, prange.s2s, false)
}

fn get_proj_mult_normal(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: ItemKey,
    a_effect: &ad::AEffect,
    prange: AttrVal,
    restricted: bool,
) -> AttrVal {
    // Assume optimal range is 0 if it's not available
    let affector_optimal = match a_effect.range_attr_id {
        Some(optimal_a_attr_id) => match calc.get_item_attr_val_full(ctx, affector_key, &optimal_a_attr_id) {
            Ok(val) => val.dogma,
            _ => OF(0.0),
        },
        None => OF(0.0),
    };
    // Assume falloff range is 0 if it's not available
    let affector_falloff = match a_effect.falloff_attr_id {
        Some(falloff_a_attr_id) => match calc.get_item_attr_val_full(ctx, affector_key, &falloff_a_attr_id) {
            Ok(val) => val.dogma,
            _ => OF(0.0),
        },
        None => OF(0.0),
    };
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
