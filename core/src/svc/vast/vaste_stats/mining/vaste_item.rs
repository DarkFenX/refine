use super::shared::get_mps_cycle_options;
use crate::{
    misc::MiningAmount,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_per_second, aggr_proj_looped_per_second},
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
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
    let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycle_options, ignore_state) {
        Some(cseq_map) => cseq_map,
        None => return item_mps,
    };
    for (effect_key, cseq) in cseq_map {
        let effect = ctx.u_data.src.get_effect(effect_key);
        let ospec = match mining_ospec_getter(&effect) {
            Some(ospec) => ospec,
            None => continue,
        };
        match cycle_options {
            CyclingOptions::Burst => {
                if let Some(effect_mps) =
                    aggr_proj_first_per_second(ctx, calc, item_key, effect, &cseq, &ospec, None, None)
                {
                    item_mps += effect_mps;
                }
            }
            CyclingOptions::Sim(_) => {
                if let Some(effect_mps) = aggr_proj_looped_per_second(ctx, calc, item_key, effect, &cseq, &ospec, None)
                {
                    item_mps += effect_mps;
                }
            }
        }
    }
    item_mps
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
