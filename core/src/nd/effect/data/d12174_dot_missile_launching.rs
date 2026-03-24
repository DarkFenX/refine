use crate::{
    ad::AEffectId,
    def::SERVER_TICK_S,
    ed::EEffectId,
    misc::Breacher,
    nd::{NEffect, NEffectDmgKind, NEffectProjMultGetter, NEffectProjOpcSpec},
    num::{Count, PValue, UnitInterval, Value},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemId},
};

const EFFECT_EID: EEffectId = EEffectId::DOT_MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::DOT_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        breacher_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_dmg_opc,
            proj_mult_chance: Some(NEffectProjMultGetter::MissileRange),
            ..
        }),
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
    _effect: &REffect,
    _base_xargs: (),
) -> Option<Output<Breacher>> {
    let abs_max = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        projector_uid,
        ctx.ac().dot_max_dmg_per_tick,
        Value::ZERO,
    )?);
    let rel_max = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, projector_uid, ctx.ac().dot_max_hp_perc_per_tick, Value::ZERO)?
            / Value::HUNDRED,
    );
    let duration_s = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, projector_uid, ctx.ac().dot_duration, Value::ZERO)? / Value::THOUSAND,
    );
    let breacher = Breacher::try_new(
        abs_max,
        UnitInterval::from_pvalue_clamped(rel_max),
        Count::from_pvalue_trunced(duration_s / PValue::from_f64_unchecked(SERVER_TICK_S)),
    )?;
    Some(Output::Simple(OutputSimple {
        instance: breacher,
        delay: PValue::ZERO,
    }))
}
