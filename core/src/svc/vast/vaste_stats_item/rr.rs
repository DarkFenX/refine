use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    misc::{EffectSpec, Spool},
    sol::REffs,
    svc::{SvcCtx, calc::Calc, efuncs, vast::Vast},
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_orr_shield(
        ctx: SvcCtx,
        calc: &mut Calc,
        reffs: &REffs,
        item_key: ItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Option<AttrVal> {
        let uad_item = ctx.uad.items.get(item_key);
        if !uad_item.is_loaded() {
            return None;
        }
        match ignore_state {
            true => {
                let a_effect_ids = uad_item.get_a_effect_datas().unwrap().keys();
                Some(get_item_orr(ctx, calc, item_key, a_effect_ids, spool))
            }
            false => {
                let a_effect_ids = reffs.iter_running(&item_key);
                Some(get_item_orr(ctx, calc, item_key, a_effect_ids, spool))
            }
        }
    }
}

fn get_item_orr<'a>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    a_effect_ids: impl Iterator<Item = &'a ad::AEffectId>,
    spool: Option<Spool>,
) -> AttrVal {
    let mut item_orr = OF(0.0);
    for a_effect_id in a_effect_ids {
        if let Some(effect_orr) = get_effect_orr(ctx, calc, item_key, a_effect_id, spool) {
            item_orr += effect_orr;
        }
    }
    item_orr
}

fn get_effect_orr(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    a_effect_id: &ad::AEffectId,
    spool: Option<Spool>,
) -> Option<AttrVal> {
    let a_effect = ctx.uad.src.get_a_effect(a_effect_id)?;
    let rep_getter = a_effect.hc.get_remote_shield_rep_amount?;
    let cycle_time = efuncs::get_effect_cycle_time(ctx, calc, item_key, a_effect)?;
    if cycle_time <= OF(0.0) {
        return None;
    }
    let rep_amount = rep_getter(ctx, calc, EffectSpec::new(item_key, a_effect.ae.id), spool, None)?;
    Some(rep_amount / cycle_time)
}
