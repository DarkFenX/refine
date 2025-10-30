use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Ecm, EffectSpec, Spool},
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{
            dmg_opc::get_dmg_opc_missile,
            proj_mult::{get_bomb_proj_mult, get_noapp_bomb_proj_mult, get_radius_ratio_mult, get_range_mult_bomb},
        },
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        eff_funcs,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemKey},
};

const E_EFFECT_ID: EEffectId = ec::effects::BOMB_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::BOMB_LAUNCHING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            dmg_kind_getter: Some(internal_get_dmg_kind),
            normal_dmg_opc_getter: Some(internal_get_dmg_opc),
            neut_opc_getter: Some(internal_get_neut_opc),
            ecm_opc_getter: Some(internal_get_ecm_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Bomb
}

fn internal_get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    get_dmg_opc_missile(
        ctx,
        calc,
        projector_key,
        projector_effect,
        projectee_key,
        get_bomb_proj_mult,
    )
}

fn internal_get_neut_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::ENERGY_NEUT_AMOUNT)?;
    // Do not return neut stats for non-neut bombs
    if amount <= OF(0.0) {
        return None;
    }
    if let Some(projectee_key) = projectee_key {
        // Here, projection reduction is split into 2 separate parts, range and application
        // reduction. This is done to correctly process cases when target has 50% chance to hit, and
        // target's cap pool is below post-application/resist bomb neut value
        amount *= get_radius_ratio_mult(ctx, calc, projector_key, projectee_key, &ac::attrs::AOE_CLOUD_SIZE);
        // Effect resistance reduction
        if let Some(resist_mult) =
            eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_effect, projectee_key)
        {
            amount *= resist_mult;
        }
        // Total resource pool limit
        if let Some(cap) = calc.get_item_attr_val_extra_opt(ctx, projectee_key, &ac::attrs::CAPACITOR_CAPACITY) {
            amount = amount.min(cap);
        }
        // Range reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.get_key()),
            projectee_key,
        );
        amount *= get_range_mult_bomb(ctx, calc, projector_key, proj_data);
    }
    Some(Output::Simple(OutputSimple { amount, delay: OF(0.0) }))
}

fn internal_get_ecm_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
) -> Option<Ecm> {
    let mut str_radar = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::SCAN_RADAR_STRENGTH_BONUS)?;
    let mut str_magnet =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::SCAN_MAGNETOMETRIC_STRENGTH_BONUS)?;
    let mut str_grav =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::SCAN_GRAVIMETRIC_STRENGTH_BONUS)?;
    let mut str_ladar = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::SCAN_LADAR_STRENGTH_BONUS)?;
    // Do not return ECM stats for non-ecm bombs
    if str_radar <= OF(0.0) && str_magnet <= OF(0.0) && str_grav <= OF(0.0) && str_ladar <= OF(0.0) {
        return None;
    }
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.get_key()),
            projectee_key,
        );
        // Lockbreaker bombs have perfect application whenever they hit, regardless of target
        // signature radius
        let mut mult = get_noapp_bomb_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        // Effect resistance reduction
        if let Some(resist_mult) =
            eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_effect, projectee_key)
        {
            mult *= resist_mult;
        }
        // Apply multiplier
        str_radar *= mult;
        str_magnet *= mult;
        str_grav *= mult;
        str_ladar *= mult;
    }
    Some(Ecm {
        radar: str_radar,
        magnetometric: str_magnet,
        gravimetric: str_grav,
        ladar: str_ladar,
        duration: OF(0.0),
    })
}
