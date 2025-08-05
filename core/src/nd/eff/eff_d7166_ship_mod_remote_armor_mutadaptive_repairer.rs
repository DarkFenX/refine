use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::{ResolvedSpool, Spool},
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
            spool::get_resolved_spool,
        },
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        eff_funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_simple_s2s),
            spool_resolver: Some(internal_get_resolved_spool),
            remote_armor_rep_opc_getter: Some(get_spool_remote_rep_opc),
            ..
        },
        ..
    }
}

fn internal_get_resolved_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &REffect,
    spool: Option<Spool>,
) -> Option<ResolvedSpool> {
    get_resolved_spool(
        ctx,
        calc,
        item_key,
        r_effect,
        spool,
        &ac::attrs::REP_MULT_BONUS_PER_CYCLE,
        &ac::attrs::REP_MULT_BONUS_MAX,
    )
}

fn get_spool_remote_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_r_effect: &REffect,
    spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::ARMOR_DMG_AMOUNT)?;
    let delay = eff_funcs::get_effect_duration_s(ctx, calc, projector_key, projector_r_effect)?;
    if let Some(resolved_spool) = internal_get_resolved_spool(ctx, calc, projector_key, projector_r_effect, spool) {
        amount *= resolved_spool.mult;
    }
    if let Some(projectee_key) = projectee_key {
        // Effect resistance reduction
        if let Some(rr_mult) =
            eff_funcs::get_effect_resist_mult(ctx, calc, projector_key, projector_r_effect, projectee_key)
        {
            amount *= rr_mult;
        }
        // Range reduction
        if let Some(proj_mult) =
            eff_funcs::get_effect_proj_mult(ctx, calc, projector_key, projector_r_effect, projectee_key)
        {
            amount *= proj_mult;
        }
        // Total resource pool limit
        if let Some(hp) = calc.get_item_attr_val_extra_opt(ctx, projectee_key, &ac::attrs::ARMOR_HP) {
            amount = amount.min(hp);
        }
    }
    Some(Output::Simple(OutputSimple { amount, delay }))
}
