use crate::{
    def::OF,
    misc::MiningAmount,
    rd::{REffectId, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::get_item_cseq_map,
        vast::{StatMining, StatMiningItemKinds, StatTimeOptions, Vast},
    },
    ud::{UFitId, UItemId},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_mps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
    ) -> StatMining {
        fit_keys
            .map(|fit_key| StatMining {
                ore: get_mps(
                    ctx,
                    calc,
                    item_kinds,
                    time_options,
                    &self.get_fit_data(&fit_key).mining_ore,
                ),
                ice: get_mps(
                    ctx,
                    calc,
                    item_kinds,
                    time_options,
                    &self.get_fit_data(&fit_key).mining_ice,
                ),
                gas: get_mps(
                    ctx,
                    calc,
                    item_kinds,
                    time_options,
                    &self.get_fit_data(&fit_key).mining_gas,
                ),
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_mps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitId,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
    ) -> StatMining {
        let fit_data = self.get_fit_data(&fit_key);
        StatMining {
            ore: get_mps(ctx, calc, item_kinds, time_options, &fit_data.mining_ore),
            ice: get_mps(ctx, calc, item_kinds, time_options, &fit_data.mining_ice),
            gas: get_mps(ctx, calc, item_kinds, time_options, &fit_data.mining_gas),
        }
    }
}

fn get_mps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatMiningItemKinds,
    time_options: StatTimeOptions,
    fit_data: &RMapRMap<UItemId, REffectId, REffectProjOpcSpec<MiningAmount>>,
) -> MiningAmount {
    let mut mps = MiningAmount::default();
    let cycling_options = time_options.into();
    for (&item_key, item_data) in fit_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        let u_item = ctx.u_data.items.get(item_key);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_mps) =
                        aggr_proj_first_ps(ctx, calc, item_key, effect, cseq, ospec, None, burst_opts.spool)
                    {
                        mps += effect_mps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_mps) =
                            aggr_proj_time_ps(ctx, calc, item_key, effect, cseq, ospec, None, time)
                        {
                            mps += effect_mps;
                        }
                    }
                    _ => {
                        if let Some(effect_mps) = aggr_proj_looped_ps(ctx, calc, item_key, effect, cseq, ospec, None) {
                            mps += effect_mps;
                        }
                    }
                },
            }
        }
    }
    mps
}
