use super::{
    application::{get_missile_application_mult, get_radius_ratio_mult, get_turret_application_mult},
    range::{
        get_aoe_burst_range_mult, get_aoe_dd_range_mult, get_dd_neut_range_mult, get_full_restricted_range_mult,
        get_full_unrestricted_range_mult, get_missile_range_mult, get_simple_c2s_range_mult, get_simple_s2s_range_mult,
    },
};
use crate::{
    def::{AttrVal, OF},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjData},
};

pub(in crate::nd::effect::data) fn get_null_proj_mult(
    _ctx: SvcCtx,
    _calc: &mut Calc,
    _projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    _proj_data: UProjData,
) -> AttrVal {
    OF(0.0)
}

pub(in crate::nd::effect::data) fn get_turret_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let mut cth = get_full_unrestricted_range_mult(ctx, calc, projector_key, projector_effect, proj_data);
    if cth == OF(0.0) {
        return OF(0.0);
    }
    cth *= get_turret_application_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
    if cth == OF(0.0) {
        return OF(0.0);
    }
    calc_turret_mult(cth)
}

pub(in crate::nd::effect::data) fn get_disintegrator_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let mut cth = get_simple_s2s_range_mult(ctx, calc, projector_key, projector_effect, proj_data);
    if cth == OF(0.0) {
        return OF(0.0);
    }
    cth *= get_turret_application_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
    if cth == OF(0.0) {
        return OF(0.0);
    }
    calc_turret_mult(cth)
}

pub(in crate::nd::effect::data) fn get_vorton_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let mult = get_simple_s2s_range_mult(ctx, calc, projector_key, projector_effect, proj_data);
    if mult == OF(0.0) {
        return OF(0.0);
    }
    mult * get_missile_application_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data)
}

pub(in crate::nd::effect::data) fn get_bubble_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_simple_c2s_range_mult(ctx, calc, projector_key, projector_effect, proj_data)
}

pub(in crate::nd::effect::data) fn get_aoe_burst_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let mult = get_aoe_burst_range_mult(ctx, calc, projector_key, projector_effect, proj_data);
    if mult == OF(0.0) {
        return OF(0.0);
    }
    mult * get_radius_ratio_mult(
        ctx,
        calc,
        projector_key,
        projectee_key,
        ctx.ac().doomsday_aoe_sig_radius,
    )
}

pub(in crate::nd::effect::data) fn get_aoe_dd_dmg_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let mult = get_aoe_dd_range_mult(ctx, calc, projector_key, proj_data);
    if mult == OF(0.0) {
        return OF(0.0);
    }
    mult * get_radius_ratio_mult(ctx, calc, projector_key, projectee_key, ctx.ac().sig_radius)
}

pub(in crate::nd::effect::data) fn get_aoe_dd_side_neut_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let mult = get_dd_neut_range_mult(ctx, calc, projector_key, proj_data);
    if mult == OF(0.0) {
        return OF(0.0);
    }
    mult * get_radius_ratio_mult(
        ctx,
        calc,
        projector_key,
        projectee_key,
        ctx.ac().doomsday_energy_neut_sig_radius,
    )
}

pub(in crate::nd::effect::data) fn get_neut_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let mult = get_full_restricted_range_mult(ctx, calc, projector_key, projector_effect, proj_data);
    if mult == OF(0.0) {
        return OF(0.0);
    }
    mult * get_radius_ratio_mult(
        ctx,
        calc,
        projector_key,
        projectee_key,
        ctx.ac().energy_neut_sig_resolution,
    )
}

// Just range projection, application factor is excluded
pub(in crate::nd::effect::data) fn get_simple_s2s_noapp_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_simple_s2s_range_mult(ctx, calc, projector_key, projector_effect, proj_data)
}

pub(in crate::nd::effect::data) fn get_full_noapp_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_full_restricted_range_mult(ctx, calc, projector_key, projector_effect, proj_data)
}

pub(in crate::nd::effect::data) fn get_aoe_burst_noapp_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_aoe_burst_range_mult(ctx, calc, projector_key, projector_effect, proj_data)
}

pub(in crate::nd::effect::data) fn get_aoe_dd_noapp_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_aoe_dd_range_mult(ctx, calc, projector_key, proj_data)
}

// Utility
fn calc_turret_mult(chance_to_hit: AttrVal) -> AttrVal {
    // https://wiki.eveuniversity.org/Turret_mechanics#Damage
    let wrecking_chance = chance_to_hit.min(OF(0.01));
    let wrecking_part = wrecking_chance * OF(3.0);
    let normal_chance = chance_to_hit - wrecking_chance;
    let normal_part = match normal_chance > OF(0.0) {
        true => {
            let avg_dmg_mult = (OF(0.01) + chance_to_hit) / OF(2.0) + OF(0.49);
            normal_chance * avg_dmg_mult
        }
        false => OF(0.0),
    };
    normal_part + wrecking_part
}
