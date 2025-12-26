use super::shared::get_mps_cycle_options;
use crate::{
    def::{AttrVal, OF},
    misc::MiningAmount,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleSeq, CyclingOptions, get_item_cseq_map},
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
    cycle_options: CyclingOptions,
    ignore_state: bool,
    mining_ospec_getter: fn(&REffect) -> Option<REffectProjOpcSpec<MiningAmount>>,
) -> MiningAmount {
    let mut item_mps = MiningAmount::default();
    let cycle_map = match get_item_cseq_map(ctx, calc, item_key, cycle_options, ignore_state) {
        Some(cycle_map) => cycle_map,
        None => return item_mps,
    };
    for (effect_key, cycle) in cycle_map {
        let effect = ctx.u_data.src.get_effect(effect_key);
        if let Some(effect_mps) = get_mps_effect(ctx, calc, item_key, effect, cycle, mining_ospec_getter) {
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
    effect_cycle: CycleSeq,
    mining_ospec_getter: fn(&REffect) -> Option<REffectProjOpcSpec<MiningAmount>>,
) -> Option<MiningAmount> {
    let ospec = mining_ospec_getter(effect)?;
    let effect_cycle_loop = effect_cycle.to_time().try_loop_cseq()?;
    let mut mining = MiningAmount {
        yield_: OF(0.0),
        drain: OF(0.0),
    };
    let mut time = OF(0.0);
    let invar_data = ospec.make_invar_data(ctx, calc, item_key, effect, None);
    for effect_cycle_part in effect_cycle_loop.iter_cseq_parts() {
        let cycle_mining = ospec.get_total(ctx, calc, item_key, effect, None, None, invar_data)?;
        mining += cycle_mining * AttrVal::from(effect_cycle_part.repeat_count);
        time += effect_cycle_part.data.time * effect_cycle_part.repeat_count as f64;
    }
    Some(mining / time)
}

fn get_getter_ore(effect: &REffect) -> Option<REffectProjOpcSpec<MiningAmount>> {
    effect.mining_ore_opc_spec
}

fn get_getter_ice(effect: &REffect) -> Option<REffectProjOpcSpec<MiningAmount>> {
    effect.mining_ice_opc_spec
}

fn get_getter_gas(effect: &REffect) -> Option<REffectProjOpcSpec<MiningAmount>> {
    effect.mining_gas_opc_spec
}
