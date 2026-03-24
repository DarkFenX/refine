use super::stat::StatMining;
use crate::{
    misc::MiningAmount,
    nd::NMiningXargs,
    num::PValue,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_first, aggr_proj_looped, aggr_proj_time},
            stats::item_checks::check_drone_module,
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_mps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        mission_ore: bool,
        ignore_state: bool,
    ) -> Result<StatMining, StatItemCheckError> {
        check_drone_module(ctx.u_data, item_uid)?;
        let base_xargs = NMiningXargs { mission_ore };
        let mps = StatMining {
            ore: get_mps_item_uid(
                ctx,
                calc,
                item_uid,
                time_options,
                base_xargs,
                ignore_state,
                get_getter_ore,
            ),
            ice: get_mps_item_uid(
                ctx,
                calc,
                item_uid,
                time_options,
                base_xargs,
                ignore_state,
                get_getter_ice,
            ),
            gas: get_mps_item_uid(
                ctx,
                calc,
                item_uid,
                time_options,
                base_xargs,
                ignore_state,
                get_getter_gas,
            ),
        };
        Ok(mps)
    }
}

fn get_mps_item_uid<F>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    time_options: StatTimeOptions,
    base_xargs: NMiningXargs,
    ignore_state: bool,
    mining_ospec_getter: F,
) -> MiningAmount
where
    F: Fn(&REffect) -> Option<&REffectProjOpcSpec<MiningAmount, NMiningXargs>>,
{
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
        let mut accum = SeqAccum::new_stack();
        if match time_options {
            StatTimeOptions::Burst(burst_opts) => aggr_proj_first(
                ctx,
                calc,
                item_uid,
                effect,
                &cseq,
                ospec,
                base_xargs,
                None,
                burst_opts.spool,
                &mut accum,
            ),
            StatTimeOptions::Sim(sim_options) => match sim_options.time {
                Some(time) if time > PValue::ZERO => aggr_proj_time(
                    ctx, calc, item_uid, effect, &cseq, ospec, base_xargs, None, &mut accum, time,
                ),
                _ => aggr_proj_looped(ctx, calc, item_uid, effect, &cseq, ospec, base_xargs, None, &mut accum),
            },
        } {
            mps += accum.get_per_second();
        }
    }
    mps
}

fn get_getter_ore(effect: &REffect) -> Option<&REffectProjOpcSpec<MiningAmount, NMiningXargs>> {
    effect.mining_ore_opc_spec.as_ref()
}

fn get_getter_ice(effect: &REffect) -> Option<&REffectProjOpcSpec<MiningAmount, NMiningXargs>> {
    effect.mining_ice_opc_spec.as_ref()
}

fn get_getter_gas(effect: &REffect) -> Option<&REffectProjOpcSpec<MiningAmount, NMiningXargs>> {
    effect.mining_gas_opc_spec.as_ref()
}
