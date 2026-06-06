use super::stat::{StatMining, StatMiningEntry};
use crate::{
    nd::{NEffectMiningAmount, NEffectMiningXargs},
    num::PValue,
    rd::{REffect, REffectMining},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_burst, aggr_proj_looped, aggr_proj_time},
            stats::item_checks::check_drone_module,
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_mps(
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        mission_ore: bool,
    ) -> Result<StatMining, StatItemCheckError> {
        check_drone_module(ctx.u_data, item_uid)?;
        let base_xargs = NEffectMiningXargs { mission_ore };
        let mps = StatMining {
            ore: get_mps_item_uid(
                reuse_cseq_map,
                ctx,
                calc,
                item_uid,
                time_options,
                base_xargs,
                get_effect_mining_ore,
            ),
            ice: get_mps_item_uid(
                reuse_cseq_map,
                ctx,
                calc,
                item_uid,
                time_options,
                base_xargs,
                get_effect_mining_ice,
            ),
            gas: get_mps_item_uid(
                reuse_cseq_map,
                ctx,
                calc,
                item_uid,
                time_options,
                base_xargs,
                get_effect_mining_gas,
            ),
        };
        Ok(mps)
    }
}

fn get_mps_item_uid<F>(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    time_options: StatTimeOptions,
    base_xargs: NEffectMiningXargs,
    mining_ospec_getter: F,
) -> StatMiningEntry
where
    F: Fn(&REffect) -> Option<&REffectMining>,
{
    let mut mps = NEffectMiningAmount::new();
    let cycling_options = CyclingOptions::from_time_options(time_options);
    if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
        return StatMiningEntry::from_effect_amount(mps);
    }
    let item = ctx.u_data.items.get(item_uid);
    for (&effect_rid, cseq) in reuse_cseq_map.iter() {
        let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
        let ospec = match mining_ospec_getter(effect) {
            Some(effect_mining) if effect_mining.check(item) => &effect_mining.ospec,
            _ => continue,
        };
        if let Some(accum) = match time_options {
            StatTimeOptions::Burst(burst_opts) => aggr_proj_burst(
                ctx,
                calc,
                item_uid,
                effect,
                cseq,
                ospec,
                base_xargs,
                None,
                burst_opts.spool,
                SeqAccum::new_stack(),
            ),
            StatTimeOptions::Sim(sim_options) => match sim_options.time {
                Some(time) if time > PValue::ZERO => aggr_proj_time(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    ospec,
                    base_xargs,
                    None,
                    SeqAccum::new_stack(),
                    time,
                ),
                _ => aggr_proj_looped(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    ospec,
                    base_xargs,
                    None,
                    SeqAccum::new_stack(),
                ),
            },
        } {
            mps += accum.get_per_second();
        }
    }
    StatMiningEntry::from_effect_amount(mps)
}

fn get_effect_mining_ore(effect: &REffect) -> Option<&REffectMining> {
    effect.mining_ore.as_ref()
}

fn get_effect_mining_ice(effect: &REffect) -> Option<&REffectMining> {
    effect.mining_ice.as_ref()
}

fn get_effect_mining_gas(effect: &REffect) -> Option<&REffectMining> {
    effect.mining_gas.as_ref()
}
