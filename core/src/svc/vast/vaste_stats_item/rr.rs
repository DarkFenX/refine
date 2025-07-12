use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    misc::{EffectSpec, Spool},
    nd::NRemoteRepGetter,
    sol::REffs,
    svc::{SvcCtx, calc::Calc, efuncs, vast::Vast},
    uad::UadItem,
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
        get_orr_item_key(ctx, calc, reffs, item_key, spool, ignore_state, get_getter_shield)
    }
    pub(in crate::svc) fn get_stat_item_orr_armor(
        ctx: SvcCtx,
        calc: &mut Calc,
        reffs: &REffs,
        item_key: ItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Option<AttrVal> {
        get_orr_item_key(ctx, calc, reffs, item_key, spool, ignore_state, get_getter_armor)
    }
    pub(in crate::svc) fn get_stat_item_orr_hull(
        ctx: SvcCtx,
        calc: &mut Calc,
        reffs: &REffs,
        item_key: ItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Option<AttrVal> {
        get_orr_item_key(ctx, calc, reffs, item_key, spool, ignore_state, get_getter_hull)
    }
    pub(in crate::svc) fn get_stat_item_orr_cap(
        ctx: SvcCtx,
        calc: &mut Calc,
        reffs: &REffs,
        item_key: ItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Option<AttrVal> {
        get_orr_item_key(ctx, calc, reffs, item_key, spool, ignore_state, get_getter_cap)
    }
}

fn item_check(uad_item: &UadItem) -> Option<()> {
    match uad_item {
        UadItem::Drone(drone) => drone.is_loaded().then_some(()),
        UadItem::Fighter(fighter) => fighter.is_loaded().then_some(()),
        UadItem::Module(module) => module.is_loaded().then_some(()),
        _ => None,
    }
}

fn get_orr_item_key(
    ctx: SvcCtx,
    calc: &mut Calc,
    reffs: &REffs,
    item_key: ItemKey,
    spool: Option<Spool>,
    ignore_state: bool,
    rep_getter_getter: fn(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter>,
) -> Option<AttrVal> {
    let uad_item = ctx.uad.items.get(item_key);
    item_check(uad_item)?;
    match ignore_state {
        true => {
            let a_effect_ids = uad_item.get_a_effect_datas().unwrap().keys();
            Some(get_orr_effect_ids(
                ctx,
                calc,
                item_key,
                a_effect_ids,
                spool,
                rep_getter_getter,
            ))
        }
        false => {
            let a_effect_ids = reffs.iter_running(&item_key);
            Some(get_orr_effect_ids(
                ctx,
                calc,
                item_key,
                a_effect_ids,
                spool,
                rep_getter_getter,
            ))
        }
    }
}

fn get_orr_effect_ids<'a>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    a_effect_ids: impl Iterator<Item = &'a ad::AEffectId>,
    spool: Option<Spool>,
    rep_getter_getter: fn(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter>,
) -> AttrVal {
    let mut item_orr = OF(0.0);
    for a_effect_id in a_effect_ids {
        if let Some(effect_orr) = get_orr_effect_id(ctx, calc, item_key, a_effect_id, spool, rep_getter_getter) {
            item_orr += effect_orr;
        }
    }
    item_orr
}

fn get_orr_effect_id(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    a_effect_id: &ad::AEffectId,
    spool: Option<Spool>,
    rep_getter_getter: fn(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter>,
) -> Option<AttrVal> {
    let a_effect = ctx.uad.src.get_a_effect(a_effect_id)?;
    let rep_getter = rep_getter_getter(a_effect)?;
    let cycle_time = efuncs::get_effect_cycle_time(ctx, calc, item_key, a_effect)?;
    if cycle_time <= OF(0.0) {
        return None;
    }
    let rep_amount = rep_getter(ctx, calc, EffectSpec::new(item_key, a_effect.ae.id), spool, None)?;
    Some(rep_amount / cycle_time)
}

fn get_getter_shield(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter> {
    a_effect_id.hc.get_remote_shield_rep_amount
}

fn get_getter_armor(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter> {
    a_effect_id.hc.get_remote_armor_rep_amount
}

fn get_getter_hull(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter> {
    a_effect_id.hc.get_remote_hull_rep_amount
}

fn get_getter_cap(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter> {
    a_effect_id.hc.get_remote_cap_rep_amount
}
