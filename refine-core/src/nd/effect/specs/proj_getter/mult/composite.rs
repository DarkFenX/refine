use super::{
    application::{
        get_legacy_missile_application_mult, get_radius_ratio_mult, get_std_missile_application_mult,
        get_turret_application_mult,
    },
    range::{
        get_aoe_burst_range_mult, get_aoe_dd_range_mult, get_dd_neut_range_mult, get_simple_s2s_range_mult,
        get_std_full_restricted_range_mult, get_std_full_unrestricted_range_mult, get_std_simple_s2s_range_mult,
    },
};
use crate::{
    num::PValue,
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemId, UProjData},
};

pub(in crate::nd::effect::specs::proj_getter) fn get_turret_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mut cth = get_std_full_unrestricted_range_mult(ctx, calc, projector_uid, effect, proj_data);
    if cth == PValue::ZERO {
        return PValue::ZERO;
    }
    cth *= get_turret_application_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
    if cth == PValue::ZERO {
        return PValue::ZERO;
    }
    calc_turret_mult(cth)
}

pub(in crate::nd::effect::specs::proj_getter) fn get_disintegrator_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mut cth = get_std_simple_s2s_range_mult(ctx, calc, projector_uid, effect, proj_data);
    if cth == PValue::ZERO {
        return PValue::ZERO;
    }
    cth *= get_turret_application_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
    if cth == PValue::ZERO {
        return PValue::ZERO;
    }
    calc_turret_mult(cth)
}

pub(in crate::nd::effect::specs::proj_getter) fn get_vorton_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_std_simple_s2s_range_mult(ctx, calc, projector_uid, effect, proj_data);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_std_missile_application_mult(ctx, calc, projector_uid, projectee_uid, proj_data)
}

pub(in crate::nd::effect::specs::proj_getter) fn get_aoe_burst_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_aoe_burst_range_mult(ctx, calc, projector_uid, proj_data);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_radius_ratio_mult(
        ctx,
        calc,
        projector_uid,
        projectee_uid,
        ctx.ac().doomsday_aoe_sig_radius,
    )
}

pub(in crate::nd::effect::specs::proj_getter) fn get_aoe_dd_dmg_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_aoe_dd_range_mult(ctx, calc, projector_uid, proj_data);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_radius_ratio_mult(ctx, calc, projector_uid, projectee_uid, ctx.ac().sig_radius)
}

pub(in crate::nd::effect::specs::proj_getter) fn get_aoe_dd_side_neut_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_dd_neut_range_mult(ctx, calc, projector_uid, proj_data);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_radius_ratio_mult(
        ctx,
        calc,
        projector_uid,
        projectee_uid,
        ctx.ac().doomsday_energy_neut_sig_radius,
    )
}

pub(in crate::nd::effect::specs::proj_getter) fn get_neut_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_std_full_restricted_range_mult(ctx, calc, projector_uid, effect, proj_data);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_radius_ratio_mult(
        ctx,
        calc,
        projector_uid,
        projectee_uid,
        ctx.ac().energy_neut_sig_resolution,
    )
}

pub(in crate::nd::effect::specs::proj_getter) fn get_ftr_abil_attack_m_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_std_full_unrestricted_range_mult(ctx, calc, projector_uid, effect, proj_data);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_legacy_missile_application_mult(
        ctx,
        calc,
        projector_uid,
        projectee_uid,
        proj_data,
        ctx.ac().ftr_abil_atk_missile_explosion_radius,
        ctx.ac().ftr_abil_atk_missile_explosion_velocity,
        ctx.ac().ftr_abil_atk_missile_reduction_factor,
        ctx.ac().ftr_abil_atk_missile_reduction_sensitivity,
    )
}

pub(in crate::nd::effect::specs::proj_getter) fn get_ftr_abil_missiles_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_std_simple_s2s_range_mult(ctx, calc, projector_uid, effect, proj_data);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_legacy_missile_application_mult(
        ctx,
        calc,
        projector_uid,
        projectee_uid,
        proj_data,
        ctx.ac().ftr_abil_missiles_explosion_radius,
        ctx.ac().ftr_abil_missiles_explosion_velocity,
        ctx.ac().ftr_abil_missiles_reduction_factor,
        ctx.ac().ftr_abil_missiles_reduction_sensitivity,
    )
}

pub(in crate::nd::effect::specs::proj_getter) fn get_ftr_abil_kamikaze_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_simple_s2s_range_mult(ctx, calc, projector_uid, proj_data, ctx.ac().ftr_abil_kamikaze_range);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_radius_ratio_mult(
        ctx,
        calc,
        projector_uid,
        projectee_uid,
        ctx.ac().ftr_abil_kamikaze_sig_radius,
    )
}

// Utility
fn calc_turret_mult(chance_to_hit: PValue) -> PValue {
    // https://wiki.eveuniversity.org/Turret_mechanics#Damage
    let wrecking_chance = chance_to_hit.into_f64().min(0.01);
    let wrecking_part = wrecking_chance * 3.0;
    let normal_chance = chance_to_hit.into_f64() - wrecking_chance;
    let normal_part = match normal_chance > 0.0 {
        true => {
            let avg_dmg_mult = (0.01 + chance_to_hit.into_f64()) / 2.0 + 0.49;
            normal_chance * avg_dmg_mult
        }
        false => 0.0,
    };
    PValue::from_f64_unchecked(normal_part + wrecking_part)
}
