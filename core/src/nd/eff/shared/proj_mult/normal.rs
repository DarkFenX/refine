use ordered_float::Float;

use super::shared::get_range;
use crate::{
    ad::{AAttrId, AEffect},
    def::{AttrVal, OF},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjData},
};

pub(in crate::nd::eff) fn get_mod_proj_attrs_normal(a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [a_effect.range_attr_id, a_effect.falloff_attr_id]
}

pub(in crate::nd::eff) fn get_proj_mult_normal_restricted(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_proj_mult_normal(
        ctx,
        calc,
        projector_key,
        projector_effect,
        proj_data.get_range_s2s(),
        true,
    )
}

pub(in crate::nd::eff) fn get_proj_mult_normal_unrestricted(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_proj_mult_normal(
        ctx,
        calc,
        projector_key,
        projector_effect,
        proj_data.get_range_s2s(),
        false,
    )
}

fn get_proj_mult_normal(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    proj_range: AttrVal,
    restricted: bool,
) -> AttrVal {
    let affector_optimal = get_range(ctx, calc, projector_key, projector_effect.get_range_attr_id());
    let affector_falloff = get_range(ctx, calc, projector_key, projector_effect.get_falloff_attr_id());
    // Calculate actual range multiplier after collecting all the data
    match affector_falloff > OF(0.0) {
        true => match restricted && proj_range > affector_optimal + OF(3.0) * affector_falloff {
            true => OF(0.0),
            false => Float::powf(
                OF(0.5),
                (Float::max(OF(0.0), proj_range - affector_optimal) / affector_falloff).powi(2),
            ),
        },
        false => match proj_range <= affector_optimal {
            true => OF(1.0),
            false => OF(0.0),
        },
    }
}
