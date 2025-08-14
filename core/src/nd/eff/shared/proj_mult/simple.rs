use super::shared::get_range;
use crate::{
    ad::{AAttrId, AEffect},
    def::{AttrVal, OF},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjData},
};

pub(crate) fn get_proj_attrs_simple(a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [a_effect.range_attr_id, None]
}

pub(crate) fn get_proj_mult_simple_s2s(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UItemKey,
    r_effect: &REffect,
    u_proj_data: UProjData,
) -> AttrVal {
    let affector_optimal = get_range(ctx, calc, affector_key, r_effect.get_range_attr_id());
    match u_proj_data.get_range_s2s() <= affector_optimal {
        true => OF(1.0),
        false => OF(0.0),
    }
}
