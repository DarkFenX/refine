use crate::{
    ac,
    ad::AEffectId,
    def::OF,
    ec,
    ed::EEffectId,
    misc::{Ecm, EffectSpec},
    nd::{NEffect, NEffectHc, effect::shared::proj_mult::get_full_noapp_proj_mult},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, eff_funcs},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::STRUCT_MOD_EFFECT_ECM;
const A_EFFECT_ID: AEffectId = ac::effects::STRUCT_MOD_EFFECT_ECM;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            ecm_opc_getter: Some(internal_get_ecm_opc),
            ..
        },
        ..
    }
}

fn internal_get_ecm_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
) -> Option<Ecm> {
    let duration_s = match projector_effect.get_duration_attr_id() {
        Some(duration_attr_id) => calc.get_item_attr_val_extra_opt(ctx, projector_key, &duration_attr_id)? / OF(1000.0),
        None => OF(0.0),
    };
    let mut str_radar = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::SCAN_RADAR_STRENGTH_BONUS)?;
    let mut str_magnet =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::SCAN_MAGNETOMETRIC_STRENGTH_BONUS)?;
    let mut str_grav =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::SCAN_GRAVIMETRIC_STRENGTH_BONUS)?;
    let mut str_ladar = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::SCAN_LADAR_STRENGTH_BONUS)?;
    if let Some(projectee_key) = projectee_key {
        let mut mult = OF(1.0);
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.get_key()),
            projectee_key,
        );
        mult *= get_full_noapp_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        // Effect resistance reduction
        if let Some(rr_mult) =
            eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_effect, projectee_key)
        {
            mult *= rr_mult;
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
        duration: duration_s,
    })
}
