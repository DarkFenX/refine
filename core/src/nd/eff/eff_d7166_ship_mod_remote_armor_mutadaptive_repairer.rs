use crate::{
    ac,
    def::{AttrVal, ItemKey, OF},
    ec,
    misc::{EffectSpec, Spool},
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
    svc::{SvcCtx, calc::Calc, efuncs, get_proj_mult, get_resist_mult},
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER),
        aid: ac::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER,
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            get_remote_armor_rep_amount: Some(get_spool_remote_rep_amount),
            ..
        },
        ..
    }
}

fn get_spool_remote_rep_amount(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    spool: Option<Spool>,
    projectee_key: Option<ItemKey>,
) -> Option<AttrVal> {
    let base_amount = calc.get_item_attr_val_extra(ctx, projector_espec.item_key, &ac::attrs::ARMOR_DMG_AMOUNT)?;
    let spool = ctx.uad.get_item_key_spool(projector_espec.item_key, spool);
    let spool_step = calc
        .get_item_attr_val_extra(ctx, projector_espec.item_key, &ac::attrs::REP_MULT_BONUS_PER_CYCLE)
        .unwrap_or(OF(0.0));
    let spool_max = calc
        .get_item_attr_val_extra(ctx, projector_espec.item_key, &ac::attrs::REP_MULT_BONUS_MAX)
        .unwrap_or(OF(0.0));
    let cycle_time = efuncs::get_espec_cycle_time(ctx, calc, projector_espec).unwrap_or(OF(0.0));
    let mut amount = match spool.resolve(spool_max, spool_step, cycle_time) {
        Some(resolved_spool) => base_amount * (OF(1.0) + resolved_spool.value),
        None => base_amount,
    };
    if let Some(projectee_key) = projectee_key {
        // Effect resistance reduction
        if let Some(rr_mult) = get_resist_mult(ctx, calc, &projector_espec, projectee_key) {
            amount *= rr_mult;
        }
        // Range reduction
        if let Some(proj_mult) = get_proj_mult(ctx, calc, projector_espec, projectee_key) {
            amount *= proj_mult;
        }
        // Total resource pool limit
        if let Some(hp) = calc.get_item_attr_val_extra(ctx, projectee_key, &ac::attrs::ARMOR_HP) {
            amount = amount.min(hp);
        }
    }
    Some(amount)
}
