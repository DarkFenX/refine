use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    misc::Spool,
    nd::NRemoteRepGetter,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        vast::{StatTank, Vast},
    },
    uad::UadItem,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_remote_rps_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<StatTank<AttrVal>, StatItemCheckError> {
        item_key_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_remote_rps_unchecked(
            ctx,
            calc,
            item_key,
            spool,
            ignore_state,
        ))
    }
    fn get_stat_item_remote_rps_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> StatTank<AttrVal> {
        StatTank {
            shield: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_shield),
            armor: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_armor),
            hull: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_hull),
        }
    }
    pub(in crate::svc) fn get_stat_item_remote_cps_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        ignore_state: bool,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_key_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_remote_cps_unchecked(
            ctx,
            calc,
            item_key,
            ignore_state,
        ))
    }
    fn get_stat_item_remote_cps_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        ignore_state: bool,
    ) -> AttrVal {
        get_orr_item_key(ctx, calc, item_key, None, ignore_state, get_getter_cap)
    }
}

pub(super) fn item_key_check(ctx: SvcCtx, item_key: ItemKey) -> Result<(), StatItemCheckError> {
    let uad_item = ctx.uad.items.get(item_key);
    let is_loaded = match uad_item {
        UadItem::Drone(drone) => drone.is_loaded(),
        UadItem::Fighter(fighter) => fighter.is_loaded(),
        UadItem::Module(module) => module.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

const RR_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    reload_optionals: false,
};

fn get_orr_item_key(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    spool: Option<Spool>,
    ignore_state: bool,
    rep_getter_getter: fn(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter>,
) -> AttrVal {
    let mut item_orr = OF(0.0);
    let cycle_map = match get_item_cycle_info(ctx, calc, item_key, RR_CYCLE_OPTIONS, ignore_state) {
        Some(cycle_map) => cycle_map,
        None => return item_orr,
    };
    for (a_effect_id, cycle) in cycle_map {
        let a_effect = ctx.uad.src.get_a_effect(&a_effect_id).unwrap();
        if let Some(effect_orr) = get_orr_effect_id(ctx, calc, item_key, a_effect, cycle, spool, rep_getter_getter) {
            item_orr += effect_orr;
        }
    }
    item_orr
}

fn get_orr_effect_id(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    a_effect: &ad::AEffectRt,
    effect_cycle: Cycle,
    spool: Option<Spool>,
    rep_getter_getter: fn(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter>,
) -> Option<AttrVal> {
    let rep_getter = rep_getter_getter(a_effect)?;
    let rep_amount = rep_getter(ctx, calc, item_key, a_effect, spool, None)?;
    Some(rep_amount.get_total() / effect_cycle.get_average_cycle_time())
}

fn get_getter_shield(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter> {
    a_effect_id.hc.get_remote_shield_rep_opc
}

fn get_getter_armor(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter> {
    a_effect_id.hc.get_remote_armor_rep_opc
}

fn get_getter_hull(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter> {
    a_effect_id.hc.get_remote_hull_rep_opc
}

fn get_getter_cap(a_effect_id: &ad::AEffectRt) -> Option<NRemoteRepGetter> {
    a_effect_id.hc.get_remote_cap_rep_opc
}
