use super::{
    option::StatMiningItemKinds,
    stat::{StatMining, StatMiningEntry},
};
use crate::{
    nd::{NEffectMiningAmount, NEffectMiningOutputGetter, NEffectMiningXargs},
    num::PValue,
    rd::{REffectId, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        vast::{
            StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_burst, aggr_proj_looped, aggr_proj_time},
        },
    },
    ud::{UFitId, UItemId},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_mps(
        &self,
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
        mission_ore: bool,
    ) -> StatMining {
        let base_xargs = NEffectMiningXargs { mission_ore };
        let mut ore = NEffectMiningAmount::new();
        let mut ice = NEffectMiningAmount::new();
        let mut gas = NEffectMiningAmount::new();
        for fit_uid in fit_uids {
            let fit_data = self.get_fit_data(&fit_uid);
            ore += get_mps(
                reuse_cseq_map,
                ctx,
                calc,
                item_kinds,
                time_options,
                base_xargs,
                &fit_data.mining_ore,
            );
            ice += get_mps(
                reuse_cseq_map,
                ctx,
                calc,
                item_kinds,
                time_options,
                base_xargs,
                &fit_data.mining_ice,
            );
            gas += get_mps(
                reuse_cseq_map,
                ctx,
                calc,
                item_kinds,
                time_options,
                base_xargs,
                &fit_data.mining_gas,
            )
        }
        StatMining {
            ore: StatMiningEntry::from_effect_amount(ore),
            ice: StatMiningEntry::from_effect_amount(ice),
            gas: StatMiningEntry::from_effect_amount(gas),
        }
    }
    pub(in crate::svc) fn get_stat_fit_mps(
        &self,
        reuse_cseq_map: &mut CseqMap,
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
            ore: StatMiningEntry::from_effect_amount(get_mps(
                reuse_cseq_map,
                ctx,
                calc,
                item_kinds,
                time_options,
                base_xargs,
                &fit_data.mining_ore,
            )),
            ice: StatMiningEntry::from_effect_amount(get_mps(
                reuse_cseq_map,
                ctx,
                calc,
                item_kinds,
                time_options,
                base_xargs,
                &fit_data.mining_ice,
            )),
            gas: StatMiningEntry::from_effect_amount(get_mps(
                reuse_cseq_map,
                ctx,
                calc,
                item_kinds,
                time_options,
                base_xargs,
                &fit_data.mining_gas,
            )),
        }
    }
}

fn get_mps(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatMiningItemKinds,
    time_options: StatTimeOptions,
    base_xargs: NEffectMiningXargs,
    fit_data: &RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectMiningOutputGetter>>,
) -> NEffectMiningAmount {
    let mut mps = NEffectMiningAmount::new();
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in fit_data.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
            continue;
        }
        let u_item = ctx.u_data.items.get(item_uid);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_rid, ospec) in item_data.iter() {
            let Some(cseq) = reuse_cseq_map.get(&effect_rid) else {
                continue;
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
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
    }
    mps
}
