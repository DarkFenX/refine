use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    svc::{SvcCtx, calc::Calc},
    uad::UadProjRange,
};

pub(crate) fn get_proj_mult_simple_s2s(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: ItemKey,
    a_effect: &ad::AEffect,
    prange: UadProjRange,
) -> AttrVal {
    // Assume optimal range is 0 if it's not available
    let affector_optimal = match a_effect.range_attr_id {
        Some(optimal_a_attr_id) => match calc.get_item_attr_val_full(ctx, affector_key, &optimal_a_attr_id) {
            Ok(val) => val.dogma,
            _ => OF(0.0),
        },
        None => OF(0.0),
    };
    match prange.s2s <= affector_optimal {
        true => OF(1.0),
        false => OF(0.0),
    }
}
