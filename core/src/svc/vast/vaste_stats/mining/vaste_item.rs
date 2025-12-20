use super::shared::get_mps_cycle_options;
use crate::{
    misc::MiningAmount,
    nd::NMiningGetter,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, CycleOptions, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatMining, Vast, vaste_stats::item_checks::check_drone_module},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_mps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        reload: bool,
        ignore_state: bool,
    ) -> Result<StatMining, StatItemCheckError> {
        check_drone_module(ctx.u_data, item_key)?;
        let cycle_options = get_mps_cycle_options(reload);
        let mps = StatMining {
            ore: get_mps_item_key(ctx, calc, item_key, cycle_options, ignore_state, get_getter_ore),
            ice: get_mps_item_key(ctx, calc, item_key, cycle_options, ignore_state, get_getter_ice),
            gas: get_mps_item_key(ctx, calc, item_key, cycle_options, ignore_state, get_getter_gas),
        };
        Ok(mps)
    }
}

fn get_mps_item_key(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    cycle_options: CycleOptions,
    ignore_state: bool,
    mining_getter_getter: fn(&REffect) -> Option<NMiningGetter>,
) -> MiningAmount {
    let mut item_mps = MiningAmount::default();
    let cycle_map = match get_item_cycle_info(ctx, calc, item_key, cycle_options, ignore_state) {
        Some(cycle_map) => cycle_map,
        None => return item_mps,
    };
    for (effect_key, cycle) in cycle_map {
        let effect = ctx.u_data.src.get_effect(effect_key);
        if let Some(effect_mps) = get_mps_effect(ctx, calc, item_key, effect, cycle, mining_getter_getter) {
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
) -> Option<MiningAmount> {
    if !effect_cycle.is_infinite() {
        return None;
    }
    let mining_getter = mining_getter_getter(effect)?;
    let mining_amount = mining_getter(ctx, calc, item_key, effect)?;
    Some(mining_amount.get_total() / effect_cycle.get_average_cycle_time())
}

fn get_getter_ore(effect: &REffect) -> Option<NMiningGetter> {
    effect.mining_ore_opc_getter
}

fn get_getter_ice(effect: &REffect) -> Option<NMiningGetter> {
    effect.mining_ice_opc_getter
}

fn get_getter_gas(effect: &REffect) -> Option<NMiningGetter> {
    effect.mining_gas_opc_getter
}
