use super::{
    application::{
        get_legacy_missile_application_mult, get_radius_ratio_mult, get_std_missile_application_mult,
        get_turret_application_mult,
    },
    range::{
        get_aoe_burst_range_mult, get_aoe_dd_round_range_mult, get_aoe_dd_sharp_range_mult, get_dd_neut_range_mult,
        get_simple_s2s_range_mult, get_std_full_restricted_range_mult, get_std_full_unrestricted_range_mult,
        get_std_simple_s2s_range_mult,
    },
};
use crate::{
    num::PValue,
    rd::REffect,
    stats::StatCritOptions,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemId, UProjData},
};

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_turret_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
    crit_options: StatCritOptions,
) -> PValue {
    let mut cth = get_std_full_unrestricted_range_mult(ctx, calc, projector_uid, effect, proj_data);
    if cth == PValue::ZERO {
        return PValue::ZERO;
    }
    cth *= get_turret_application_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
    if cth == PValue::ZERO {
        return PValue::ZERO;
    }
    get_turret_mult(cth, crit_options)
}

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_disintegrator_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
    crit_options: StatCritOptions,
) -> PValue {
    let mut cth = get_std_simple_s2s_range_mult(ctx, calc, projector_uid, effect, proj_data);
    if cth == PValue::ZERO {
        return PValue::ZERO;
    }
    cth *= get_turret_application_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data);
    if cth == PValue::ZERO {
        return PValue::ZERO;
    }
    get_turret_mult(cth, crit_options)
}

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_vorton_proj_mult(
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

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_aoe_burst_proj_mult(
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

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_aoe_dd_dmg_sharp_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_aoe_dd_sharp_range_mult(ctx, calc, projector_uid, proj_data);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_radius_ratio_mult(ctx, calc, projector_uid, projectee_uid, ctx.ac().sig_radius)
}

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_aoe_dd_dmg_round_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let mult = get_aoe_dd_round_range_mult(ctx, calc, projector_uid, proj_data);
    if mult == PValue::ZERO {
        return PValue::ZERO;
    }
    mult * get_radius_ratio_mult(ctx, calc, projector_uid, projectee_uid, ctx.ac().sig_radius)
}

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_aoe_dd_side_neut_proj_mult(
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

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_neut_proj_mult(
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

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_ftr_abil_attack_m_proj_mult(
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

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_ftr_abil_missiles_proj_mult(
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

pub(in crate::nd::effect::specs::proj_mult_getter) fn get_ftr_abil_kamikaze_proj_mult(
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
pub(in crate::nd::effect::specs::proj_mult_getter) const TURRET_CTH_1_CRIT_MULT: PValue =
    PValue::from_f64_unchecked(calc_turret_mult(1.0, StatCritOptions::Include));
const TURRET_CTH_1_NO_CRIT_MULT: PValue = PValue::from_f64_unchecked(calc_turret_mult(1.0, StatCritOptions::Exclude));

fn get_turret_mult(chance_to_hit: PValue, crit_options: StatCritOptions) -> PValue {
    let chance_to_hit = match chance_to_hit {
        PValue::ONE => {
            return match crit_options {
                StatCritOptions::Include => TURRET_CTH_1_CRIT_MULT,
                StatCritOptions::Exclude => TURRET_CTH_1_NO_CRIT_MULT,
            };
        }
        _ => chance_to_hit.into_f64(),
    };
    PValue::from_f64_unchecked(calc_turret_mult(chance_to_hit, crit_options))
}

const NO_CRIT_C1: f64 = 99.0 / 199.0;
const NO_CRIT_C2: f64 = 100.0 / 199.0;

const fn calc_turret_mult(chance_to_hit: f64, crit_options: StatCritOptions) -> f64 {
    match crit_options {
        // Optimized variant of formula from https://wiki.eveuniversity.org/Turret_mechanics#Damage
        StatCritOptions::Include => f64::min(
            f64::mul_add(
                0.5 * chance_to_hit,
                chance_to_hit,
                f64::mul_add(0.49, chance_to_hit, 0.02505),
            ),
            3.0 * chance_to_hit,
        ),
        // Same formula as from the wiki, but crit part is taken out, and normal growth is stretched
        // over range of [0, 1]. Stretching is necessary to get to 100% of paper damage in case of
        // 100% chance to hit
        StatCritOptions::Exclude => f64::mul_add(chance_to_hit * chance_to_hit, NO_CRIT_C1, chance_to_hit * NO_CRIT_C2),
    }
}
