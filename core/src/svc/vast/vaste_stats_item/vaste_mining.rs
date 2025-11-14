use super::checks::check_item_key_drone_module;
use crate::{
    nd::NMiningGetter,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatMining, StatMiningAmount, Vast},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_mps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        ignore_state: bool,
    ) -> Result<StatMining, StatItemCheckError> {
        check_item_key_drone_module(ctx, item_key)?;
        Ok(Vast::get_stat_item_mps_unchecked(ctx, calc, item_key, ignore_state))
    }
    fn get_stat_item_mps_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, ignore_state: bool) -> StatMining {
        StatMining {
            ore: get_mps_item_key(ctx, calc, item_key, ignore_state, get_getter_ore),
            ice: get_mps_item_key(ctx, calc, item_key, ignore_state, get_getter_ice),
            gas: get_mps_item_key(ctx, calc, item_key, ignore_state, get_getter_gas),
        }
    }
}

const MINING_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Sim,
    reload_optionals: true,
};

fn get_mps_item_key(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    ignore_state: bool,
    mining_getter_getter: fn(&REffect) -> Option<NMiningGetter>,
) -> StatMiningAmount {
    let mut item_mps = StatMiningAmount::default();
    let cycle_map = match get_item_cycle_info(ctx, calc, item_key, MINING_CYCLE_OPTIONS, ignore_state) {
        Some(cycle_map) => cycle_map,
        None => return item_mps,
    };
    for (effect_key, cycle) in cycle_map {
        let r_effect = ctx.u_data.src.get_effect(effect_key);
        if let Some(effect_mps) = get_mps_effect(ctx, calc, item_key, r_effect, cycle, mining_getter_getter) {
            item_mps += effect_mps;
        }
    }
    item_mps
}

fn get_mps_effect(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    effect_cycle: Cycle,
    mining_getter_getter: fn(&REffect) -> Option<NMiningGetter>,
) -> Option<StatMiningAmount> {
    if !effect_cycle.is_infinite() {
        return None;
    }
    let mining_getter = mining_getter_getter(effect)?;
    let mining_amount = mining_getter(ctx, calc, item_key)?;
    Some(mining_amount.get_total() / effect_cycle.get_average_cycle_time())
}

fn get_getter_ore(effect: &REffect) -> Option<NMiningGetter> {
    effect.get_mining_ore_opc_getter()
}

fn get_getter_ice(effect: &REffect) -> Option<NMiningGetter> {
    effect.get_mining_ice_opc_getter()
}

fn get_getter_gas(effect: &REffect) -> Option<NMiningGetter> {
    effect.get_mining_gas_opc_getter()
}
