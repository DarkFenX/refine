use super::checks::check_item_key_drone_fighter_module;
use crate::{
    def::{AttrVal, OF},
    misc::Spool,
    nd::NRemoteRepGetter,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatTank, Vast},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_remote_rps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<StatTank<AttrVal>, StatItemCheckError> {
        check_item_key_drone_fighter_module(ctx, item_key)?;
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
        item_key: UItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> StatTank<AttrVal> {
        StatTank {
            shield: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_shield),
            armor: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_armor),
            hull: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_hull),
        }
    }
    pub(in crate::svc) fn get_stat_item_remote_cps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        ignore_state: bool,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_item_key_drone_fighter_module(ctx, item_key)?;
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
        item_key: UItemKey,
        ignore_state: bool,
    ) -> AttrVal {
        get_orr_item_key(ctx, calc, item_key, None, ignore_state, get_getter_cap)
    }
}

const RR_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    charged_optionals: false,
};

fn get_orr_item_key(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    spool: Option<Spool>,
    ignore_state: bool,
    rep_getter_getter: fn(&REffect) -> Option<NRemoteRepGetter>,
) -> AttrVal {
    let mut item_orr = OF(0.0);
    let cycle_map = match get_item_cycle_info(ctx, calc, item_key, RR_CYCLE_OPTIONS, ignore_state) {
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
    rep_getter_getter: fn(&REffect) -> Option<NRemoteRepGetter>,
) -> Option<AttrVal> {
    if !effect_cycle.is_infinite() {
        return None;
    }
    let rep_getter = rep_getter_getter(effect)?;
    let rep_amount = rep_getter(ctx, calc, item_key, effect, spool, None)?;
    Some(rep_amount.get_total() / effect_cycle.get_average_cycle_time())
}

fn get_getter_shield(effect: &REffect) -> Option<NRemoteRepGetter> {
    effect.get_remote_shield_rep_opc_getter()
}

fn get_getter_armor(effect: &REffect) -> Option<NRemoteRepGetter> {
    effect.get_remote_armor_rep_opc_getter()
}

fn get_getter_hull(effect: &REffect) -> Option<NRemoteRepGetter> {
    effect.get_remote_hull_rep_opc_getter()
}

fn get_getter_cap(effect_id: &REffect) -> Option<NRemoteRepGetter> {
    effect_id.get_remote_cap_rep_opc_getter()
}
