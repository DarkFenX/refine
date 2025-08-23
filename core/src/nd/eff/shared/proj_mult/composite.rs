use super::{
    application::get_application_mult_missile,
    range::{
        get_range_mult_aoe_burst, get_range_mult_bomb, get_range_mult_full_restricted,
        get_range_mult_full_unrestricted, get_range_mult_missile, get_range_mult_simple_c2s, get_range_mult_simple_s2s,
    },
};
use crate::{
    def::{AttrVal, OF},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjData},
};

pub(in crate::nd::eff) fn get_null_proj_mult(
    _ctx: SvcCtx,
    _calc: &mut Calc,
    _projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    _proj_data: UProjData,
) -> AttrVal {
    OF(0.0)
}

pub(in crate::nd::eff) fn get_noapp_simple_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_simple_s2s(ctx, calc, projector_key, projector_effect, proj_data)
}

pub(in crate::nd::eff) fn get_noapp_full_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_full_restricted(ctx, calc, projector_key, projector_effect, proj_data)
}

pub(in crate::nd::eff) fn get_turret_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_full_unrestricted(ctx, calc, projector_key, projector_effect, proj_data)
}

pub(in crate::nd::eff) fn get_disintegrator_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_simple_s2s(ctx, calc, projector_key, projector_effect, proj_data)
}

pub(in crate::nd::eff) fn get_vorton_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_simple_s2s(ctx, calc, projector_key, projector_effect, proj_data)
        * get_application_mult_missile(ctx, calc, projector_key, projectee_key, proj_data)
}

pub(in crate::nd::eff) fn get_missile_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_missile(ctx, calc, projector_key, proj_data)
        * get_application_mult_missile(ctx, calc, projector_key, projectee_key, proj_data)
}

pub(in crate::nd::eff) fn get_breacher_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_missile(ctx, calc, projector_key, proj_data)
}

pub(in crate::nd::eff) fn get_bomb_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_bomb(ctx, calc, projector_key, proj_data)
}

pub(in crate::nd::eff) fn get_bubble_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_simple_c2s(ctx, calc, projector_key, projector_effect, proj_data)
}

pub(in crate::nd::eff) fn get_aoe_burst_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_aoe_burst(ctx, calc, projector_key, projector_effect, proj_data)
}
