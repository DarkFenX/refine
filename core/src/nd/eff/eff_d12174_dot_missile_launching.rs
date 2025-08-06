use crate::{
    ac,
    ad::AEffectId,
    def::{Count, OF, SERVER_TICK_S},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_missile, get_proj_mult_missile},
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputDmgBreacher, OutputSimple},
    },
    ud::UItemKey,
    util::trunc_unerr,
};

const E_EFFECT_ID: EEffectId = ec::effects::DOT_MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::DOT_MISSILE_LAUNCHING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_missile),
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_missile),
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
    _projector_r_effect: &REffect,
    _projectee_key: Option<UItemKey>,
) -> Option<Output<OutputDmgBreacher>> {
    let abs_max = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOT_MAX_DMG_PER_TICK)?;
    let rel_max =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOT_MAX_HP_PERC_PER_TICK)? / OF(100.0);
    let duration_s = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOT_DURATION)? / OF(1000.0);
    Some(Output::Simple(OutputSimple {
        amount: OutputDmgBreacher {
            absolute_max: abs_max,
            relative_max: rel_max,
            // Breachers deal damage every tick, so count of instances depends on tick rate
            instance_count: trunc_unerr(duration_s / SERVER_TICK_S) as Count,
        },
        delay: OF(0.0),
    }))
}
