use crate::{
    ac, ad,
    def::{AttrVal, ItemKey, OF},
    ec,
    misc::{ResolvedSpool, Spool},
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
    svc::{
        SvcCtx,
        calc::Calc,
        efuncs,
        output::{Output, OutputSimple},
    },
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER),
        aid: ac::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER,
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            get_resolved_spool: Some(get_resolved_spool),
            get_remote_armor_rep_opc: Some(get_spool_remote_rep_opc),
            ..
        },
        ..
    }
}

fn get_resolved_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    a_effect: &ad::AEffectRt,
    spool: Option<Spool>,
) -> Option<ResolvedSpool> {
    let duration_s = efuncs::get_effect_duration_s(ctx, calc, item_key, a_effect)?;
    let spool = ctx.uad.get_item_key_spool(item_key, spool);
    let spool_step = calc
        .get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::REP_MULT_BONUS_PER_CYCLE)
        .unwrap_or(OF(0.0));
    let spool_max = calc
        .get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::REP_MULT_BONUS_MAX)
        .unwrap_or(OF(0.0));
    spool.resolve(spool_max, spool_step, duration_s)
}

fn get_spool_remote_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: ItemKey,
    projector_a_effect: &ad::AEffectRt,
    spool: Option<Spool>,
    projectee_key: Option<ItemKey>,
) -> Option<Output<AttrVal>> {
    let mut amount = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::ARMOR_DMG_AMOUNT)?;
    let delay = efuncs::get_effect_duration_s(ctx, calc, projector_key, projector_a_effect)?;
    if let Some(resolved_spool) = get_resolved_spool(ctx, calc, projector_key, projector_a_effect, spool) {
        amount *= resolved_spool.mult;
    }
    if let Some(projectee_key) = projectee_key {
        // Effect resistance reduction
        if let Some(rr_mult) =
            efuncs::get_effect_resist_mult(ctx, calc, projector_key, projector_a_effect, projectee_key)
        {
            amount *= rr_mult;
        }
        // Range reduction
        if let Some(proj_mult) =
            efuncs::get_effect_proj_mult(ctx, calc, projector_key, projector_a_effect, projectee_key)
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
