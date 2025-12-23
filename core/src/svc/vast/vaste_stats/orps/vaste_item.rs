use super::shared::get_orps_cycle_options;
use crate::{
    def::{AttrVal, OF},
    misc::Spool,
    nd::NOutgoingRepGetter,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatTank, Vast, vaste_stats::item_checks::check_drone_fighter_module},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_rps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<StatTank<AttrVal>, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_key)?;
        let orps = StatTank {
            shield: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_shield),
            armor: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_armor),
            hull: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_hull),
        };
        Ok(orps)
    }
    pub(in crate::svc) fn get_stat_item_outgoing_cps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        ignore_state: bool,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_key)?;
        let ocps = get_orr_item_key(ctx, calc, item_key, None, ignore_state, get_getter_cap);
        Ok(ocps)
    }
}

fn get_orr_item_key(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    spool: Option<Spool>,
    ignore_state: bool,
    rep_getter_getter: fn(&REffect) -> Option<NOutgoingRepGetter>,
) -> AttrVal {
    let mut item_orr = OF(0.0);
    // TODO: allow configuring cycle options by caller
    let cycle_options = get_orps_cycle_options(false);
    let cycle_map = match get_item_cycle_info(ctx, calc, item_key, cycle_options, ignore_state) {
        Some(cycle_map) => cycle_map,
        None => return item_orr,
    };
    for (effect_key, cycle) in cycle_map {
        let r_effect = ctx.u_data.src.get_effect(effect_key);
        if let Some(effect_orr) = get_orr_effect(ctx, calc, item_key, r_effect, cycle, spool, rep_getter_getter) {
            item_orr += effect_orr;
        }
    }
    item_orr
}

fn get_orr_effect(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    effect_cycle: Cycle,
    spool: Option<Spool>,
    rep_getter_getter: fn(&REffect) -> Option<NOutgoingRepGetter>,
) -> Option<AttrVal> {
    let rep_getter = rep_getter_getter(effect)?;
    let effect_cycle_loop = effect_cycle.try_get_loop()?;
    let rep_amount = rep_getter(ctx, calc, item_key, effect, spool, None)?;
    Some(rep_amount.get_total() / effect_cycle_loop.get_average_time())
}

fn get_getter_shield(effect: &REffect) -> Option<NOutgoingRepGetter> {
    effect.outgoing_shield_rep_opc_getter
}

fn get_getter_armor(effect: &REffect) -> Option<NOutgoingRepGetter> {
    effect.outgoing_armor_rep_opc_getter
}

fn get_getter_hull(effect: &REffect) -> Option<NOutgoingRepGetter> {
    effect.outgoing_hull_rep_opc_getter
}

fn get_getter_cap(effect_id: &REffect) -> Option<NOutgoingRepGetter> {
    effect_id.outgoing_cap_rep_opc_getter
}
