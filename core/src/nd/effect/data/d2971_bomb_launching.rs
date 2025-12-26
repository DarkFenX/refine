use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{Ecm, EffectSpec},
    nd::{
        NEffect, NEffectDmgKind, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::{
            base_opc::get_instant_dmg_base_opc,
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
    ud::{UItem, UItemKey, UProjData},
};

const E_EFFECT_ID: EEffectId = ec::effects::BOMB_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::BOMB_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_dmg_base_opc,
            proj_mult_str: Some(get_bomb_proj_mult),
            ..
        }),
        neut_opc_spec: Some(NEffectProjOpcSpec {
            base: internal_get_neut_base_opc,
            proj_mult_str: Some(internal_get_application_mult),
            proj_mult_chance: Some(internal_get_range_mult),
            resist: Some(NEffectResist::Standard),
            ilimit_attr_id: Some(ac::attrs::CAPACITOR_CAPACITY),
            ..
        }),
        ecm_opc_getter: Some(internal_get_ecm_opc),
        ..
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Damage
////////////////////////////////////////////////////////////////////////////////////////////////////
fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Bomb
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Neut
////////////////////////////////////////////////////////////////////////////////////////////////////
fn internal_get_neut_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<AttrVal>> {
    let amount = calc.get_item_oattr_afb_odogma(ctx, item_key, ctx.ac().energy_neut_amount, OF(0.0))?;
    // Do not return neut stats for non-neut bombs
    if amount <= OF(0.0) {
        return None;
    }
    Some(Output::Simple(OutputSimple { amount, delay: OF(0.0) }))
}

fn internal_get_application_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    projectee_key: UItemKey,
    _proj_data: UProjData,
) -> AttrVal {
    get_radius_ratio_mult(ctx, calc, projector_key, projectee_key, ctx.ac().aoe_cloud_size)
}

fn internal_get_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_bomb_range_mult(ctx, calc, projector_key, proj_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ECM
////////////////////////////////////////////////////////////////////////////////////////////////////
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
