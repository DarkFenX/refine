use super::{
    application::{get_application_mult_bomb, get_application_mult_missile, get_application_mult_turret},
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

pub(in crate::nd::eff) fn get_turret_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let mut cth = get_range_mult_full_unrestricted(ctx, calc, projector_key, projector_effect, proj_data);
    if cth == OF(0.0) {
        return OF(0.0);
    }
    cth *= get_application_mult_turret(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
    if cth == OF(0.0) {
        return OF(0.0);
    }
    calc_turret_mult(cth)
}

pub(in crate::nd::eff) fn get_disintegrator_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let mut cth = get_range_mult_simple_s2s(ctx, calc, projector_key, projector_effect, proj_data);
    if cth == OF(0.0) {
        return OF(0.0);
    }
    cth *= get_application_mult_turret(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
    if cth == OF(0.0) {
        return OF(0.0);
    }
    calc_turret_mult(cth)
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
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_bomb(ctx, calc, projector_key, proj_data)
        * get_application_mult_bomb(ctx, calc, projector_key, projectee_key)
}

pub(in crate::nd::eff) fn get_guided_bomb_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_missile(ctx, calc, projector_key, proj_data)
        * get_application_mult_bomb(ctx, calc, projector_key, projectee_key)
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

// Just range projection, application factor is excluded
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

pub(in crate::nd::eff) fn get_noapp_bomb_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_range_mult_bomb(ctx, calc, projector_key, proj_data)
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
