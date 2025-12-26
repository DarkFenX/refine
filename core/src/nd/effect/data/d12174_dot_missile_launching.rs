use crate::{
    ac,
    ad::AEffectId,
    def::{Count, OF, SERVER_TICK_S},
    ec,
    ed::EEffectId,
    misc::EffectSpec,
    nd::{NEffect, NEffectDmgKind, effect::data::shared::proj_mult::get_missile_range_mult},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::OutputDmgBreacher},
    ud::{UItem, UItemKey},
    util::trunc_unerr,
};

const E_EFFECT_ID: EEffectId = ec::effects::DOT_MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::DOT_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        breacher_dmg_opc_getter: Some(get_dmg_opc),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Breacher
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
) -> Option<OutputDmgBreacher> {
    let attr_consts = ctx.ac();
    let mut abs_max = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.dot_max_dmg_per_tick, OF(0.0))?;
    let mut rel_max =
        calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.dot_max_hp_perc_per_tick, OF(0.0))? / OF(100.0);
    let duration_s =
        calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.dot_duration, OF(0.0))? / OF(1000.0);
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        let mult = get_missile_range_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        abs_max *= mult;
        rel_max *= mult;
    }
    OutputDmgBreacher::new(
        abs_max,
        rel_max,
        trunc_unerr(duration_s / SERVER_TICK_S).into_inner() as Count,
    )
}
