use super::stat::StatMining;
use crate::{
    misc::MiningAmount,
    num::PValue,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatTimeOptions, Vast, stats::item_checks::check_drone_module},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_mps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        ignore_state: bool,
    ) -> Result<StatMining, StatItemCheckError> {
        check_drone_module(ctx.u_data, item_uid)?;
        let mps = StatMining {
            ore: get_mps_item_uid(ctx, calc, item_uid, time_options, ignore_state, get_getter_ore),
            ice: get_mps_item_uid(ctx, calc, item_uid, time_options, ignore_state, get_getter_ice),
            gas: get_mps_item_uid(ctx, calc, item_uid, time_options, ignore_state, get_getter_gas),
        };
        Ok(mps)
    }
}

fn get_mps_item_uid(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    time_options: StatTimeOptions,
    ignore_state: bool,
    mining_ospec_getter: fn(&REffect) -> Option<REffectProjOpcSpec<MiningAmount>>,
) -> MiningAmount {
    let mut mps = MiningAmount::default();
    let cycling_options = CyclingOptions::from_time_options(time_options);
    let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, ignore_state) {
        Some(cseq_map) => cseq_map,
        None => return mps,
    };
    for (effect_rid, cseq) in cseq_map {
        let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
        let ospec = match mining_ospec_getter(&effect) {
            Some(ospec) => ospec,
            None => continue,
        };
        match time_options {
            StatTimeOptions::Burst(burst_opts) => {
                if let Some(effect_mps) =
                    aggr_proj_first_ps(ctx, calc, item_uid, effect, &cseq, &ospec, None, burst_opts.spool)
                {
                    mps += effect_mps;
                }
            }
            StatTimeOptions::Sim(sim_options) => match sim_options.time {
                Some(time) if time > PValue::ZERO => {
                    if let Some(effect_mps) = aggr_proj_time_ps(ctx, calc, item_uid, effect, &cseq, &ospec, None, time)
                    {
                        mps += effect_mps;
                    }
                }
                _ => {
                    if let Some(effect_mps) = aggr_proj_looped_ps(ctx, calc, item_uid, effect, &cseq, &ospec, None) {
                        mps += effect_mps;
                    }
                }
            },
        }
    }
    mps
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
