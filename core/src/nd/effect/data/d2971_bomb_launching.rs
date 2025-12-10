use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Ecm, EffectSpec, Spool},
    nd::{
        NEffect, NEffectDmgKind,
        effect::data::shared::{
            opc::get_missile_dmg_opc,
            proj_mult::{get_bomb_noapp_proj_mult, get_bomb_proj_mult, get_bomb_range_mult, get_radius_ratio_mult},
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

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_getter: Some(internal_get_dmg_opc),
        neut_opc_getter: Some(internal_get_neut_opc),
        ecm_opc_getter: Some(internal_get_ecm_opc),
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
    get_missile_dmg_opc(
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
    let attr_consts = ctx.ac();
    let mut amount = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.energy_neut_amount, OF(0.0))?;
    // Do not return neut stats for non-neut bombs
    if amount <= OF(0.0) {
        return None;
    }
    if let Some(projectee_key) = projectee_key {
        let attr_consts = ctx.ac();
        // Here, projection reduction is split into 2 separate parts, range and application
        // reduction. This is done to correctly process cases when target has 50% chance to hit, and
        // target's cap pool is below post-application/resist bomb neut value
        amount *= get_radius_ratio_mult(ctx, calc, projector_key, projectee_key, attr_consts.aoe_cloud_size);
        // Effect resistance reduction
        if let Some(resist_mult) =
            eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_effect, projectee_key)
        {
            amount *= resist_mult;
        }
        // Total resource pool limit
        if let Some(cap) = calc.get_item_oattr_oextra(ctx, projectee_key, attr_consts.capacitor_capacity) {
            amount = amount.min(cap);
        }
        // Range reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        amount *= get_bomb_range_mult(ctx, calc, projector_key, proj_data);
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
    let attr_consts = ctx.ac();
    let mut str_radar =
        calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.scan_radar_strength_bonus, OF(0.0))?;
    let mut str_magnet = calc.get_item_oattr_afb_oextra(
        ctx,
        projector_key,
        attr_consts.scan_magnetometric_strength_bonus,
        OF(0.0),
    )?;
    let mut str_grav =
        calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.scan_gravimetric_strength_bonus, OF(0.0))?;
    let mut str_ladar =
        calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.scan_ladar_strength_bonus, OF(0.0))?;
    // Do not return ECM stats for non-ecm bombs
    if str_radar <= OF(0.0) && str_magnet <= OF(0.0) && str_grav <= OF(0.0) && str_ladar <= OF(0.0) {
        return None;
    }
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        // Lockbreaker bombs have perfect application whenever they hit, regardless of target
        // signature radius
        let mut mult = get_bomb_noapp_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
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
