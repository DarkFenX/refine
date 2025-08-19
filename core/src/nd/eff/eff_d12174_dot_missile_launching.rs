use crate::{
    ac,
    ad::AEffectId,
    def::{Count, OF, SERVER_TICK_S},
    ec,
    ed::EEffectId,
    misc::EffectSpec,
    nd::{NEffect, NEffectDmgKind, NEffectHc, eff::shared::proj_mult::get_proj_mult_missile},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::OutputDmgBreacher},
    ud::UItemKey,
    util::trunc_unerr,
};

const E_EFFECT_ID: EEffectId = ec::effects::DOT_MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::DOT_MISSILE_LAUNCHING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            dmg_kind: Some(NEffectDmgKind::Breacher),
            breacher_dmg_opc_getter: Some(get_dmg_opc),
            ..
        },
        ..
    }
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_r_effect: &REffect,
    projectee_key: Option<UItemKey>,
) -> Option<OutputDmgBreacher> {
    let mut abs_max = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOT_MAX_DMG_PER_TICK)?;
    let mut rel_max =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOT_MAX_HP_PERC_PER_TICK)? / OF(100.0);
    let duration_s = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOT_DURATION)? / OF(1000.0);
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let u_proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_r_effect.get_key()),
            projectee_key,
        );
        let mult = get_proj_mult_missile(ctx, calc, projector_key, projector_r_effect, u_proj_data);
        abs_max *= mult;
        rel_max *= mult;
    }
    OutputDmgBreacher::new(
        abs_max,
        rel_max,
        trunc_unerr(duration_s / SERVER_TICK_S).into_inner() as Count,
    )
}
