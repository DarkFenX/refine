use super::{option::StatMiningItemKinds, stat::StatMining};
use crate::{
    misc::MiningAmount,
    nd::{NEffectMiningOutputGetter, NEffectMiningXargs},
    num::PValue,
    rd::{REffectId, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        vast::{
            StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_first, aggr_proj_looped, aggr_proj_time},
        },
    },
    ud::{UFitId, UItemId},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_mps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
        mission_ore: bool,
    ) -> StatMining {
        let base_xargs = NEffectMiningXargs { mission_ore };
        fit_uids
            .map(|fit_uid| StatMining {
                ore: get_mps(
                    ctx,
                    calc,
                    item_kinds,
                    time_options,
                    base_xargs,
                    &self.get_fit_data(&fit_uid).mining_ore,
                ),
                ice: get_mps(
                    ctx,
                    calc,
                    item_kinds,
                    time_options,
                    base_xargs,
                    &self.get_fit_data(&fit_uid).mining_ice,
                ),
                gas: get_mps(
                    ctx,
                    calc,
                    item_kinds,
                    time_options,
                    base_xargs,
                    &self.get_fit_data(&fit_uid).mining_gas,
                ),
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_mps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
        mission_ore: bool,
    ) -> StatMining {
        let fit_data = self.get_fit_data(&fit_uid);
        let base_xargs = NEffectMiningXargs { mission_ore };
        StatMining {
            ore: get_mps(ctx, calc, item_kinds, time_options, base_xargs, &fit_data.mining_ore),
            ice: get_mps(ctx, calc, item_kinds, time_options, base_xargs, &fit_data.mining_ice),
            gas: get_mps(ctx, calc, item_kinds, time_options, base_xargs, &fit_data.mining_gas),
        }
    }
}

fn get_mps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatMiningItemKinds,
    time_options: StatTimeOptions,
    base_xargs: NEffectMiningXargs,
    fit_data: &RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectMiningOutputGetter>>,
) -> MiningAmount {
    let mut mps = MiningAmount::default();
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in fit_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        let u_item = ctx.u_data.items.get(item_uid);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let mut accum = SeqAccum::new_stack();
            if match time_options {
                StatTimeOptions::Burst(burst_opts) => aggr_proj_first(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    ospec,
                    base_xargs,
                    None,
                    burst_opts.spool,
                    &mut accum,
                ),
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => aggr_proj_time(
                        ctx, calc, item_uid, effect, cseq, ospec, base_xargs, None, &mut accum, time,
                    ),
                    _ => aggr_proj_looped(ctx, calc, item_uid, effect, cseq, ospec, base_xargs, None, &mut accum),
                },
            } {
                mps += accum.get_per_second();
            }
        }
    }
    mps
}
