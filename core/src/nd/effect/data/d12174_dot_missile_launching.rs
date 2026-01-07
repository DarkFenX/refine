use crate::{
    ad::AEffectId,
    def::SERVER_TICK_S,
    ed::EEffectId,
    misc::{Count, EffectSpec, PValue, UnitInterval, Value},
    nd::{NEffect, NEffectDmgKind, effect::data::shared::proj_mult::get_missile_range_mult},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::OutputDmgBreacher},
    ud::{UItem, UItemId},
};

const EFFECT_EID: EEffectId = EEffectId::DOT_MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::DOT_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
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
    projector_uid: UItemId,
    projector_effect: &REffect,
    projectee_uid: Option<UItemId>,
) -> Option<OutputDmgBreacher> {
    let mut abs_max = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        projector_uid,
        ctx.ac().dot_max_dmg_per_tick,
        Value::ZERO,
    )?);
    let mut rel_max = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, projector_uid, ctx.ac().dot_max_hp_perc_per_tick, Value::ZERO)?
            / Value::HUNDRED,
    );
    let duration_s = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, projector_uid, ctx.ac().dot_duration, Value::ZERO)? / Value::THOUSAND,
    );
    if let Some(projectee_key) = projectee_uid {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_uid, projector_effect.rid),
            projectee_key,
        );
        let mult = get_missile_range_mult(ctx, calc, projector_uid, projector_effect, projectee_key, proj_data);
        abs_max *= mult;
        rel_max *= mult;
    }
    OutputDmgBreacher::new(
        abs_max,
        UnitInterval::from_pvalue_clamped(rel_max),
        Count::from_pvalue_trunced(duration_s / PValue::from_f64_unchecked(SERVER_TICK_S)),
    )
}
