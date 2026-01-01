use crate::{
    def::{AttrVal, OF},
    rd::{REffectKey, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::get_item_cseq_map,
        vast::{StatNeutItemKinds, StatTimeOptions, Vast},
    },
    ud::{UFitKey, UItemKey},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_outgoing_nps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemKey>,
    ) -> AttrVal {
        fit_keys
            .map(|fit_key| {
                get_nps(
                    ctx,
                    calc,
                    item_kinds,
                    time_options,
                    projectee_key,
                    &self.get_fit_data(&fit_key).out_neuts,
                )
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_nps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemKey>,
    ) -> AttrVal {
        let fit_data = self.get_fit_data(&fit_key);
        get_nps(ctx, calc, item_kinds, time_options, projectee_key, &fit_data.out_neuts)
    }
}

fn get_nps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatNeutItemKinds,
    time_options: StatTimeOptions,
    projectee_item_key: Option<UItemKey>,
    fit_data: &RMapRMap<UItemKey, REffectKey, REffectProjOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut nps = OF(0.0);
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
                    if let Some(effect_nps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        item_key,
                        effect,
                        cseq,
                        ospec,
                        projectee_item_key,
                        burst_opts.spool,
                    ) {
                        nps += effect_nps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_nps) =
                            aggr_proj_time_ps(ctx, calc, item_key, effect, cseq, ospec, projectee_item_key, time)
                        {
                            nps += effect_nps;
                        }
                    }
                    _ => {
                        if let Some(effect_nps) =
                            aggr_proj_looped_ps(ctx, calc, item_key, effect, cseq, ospec, projectee_item_key)
                        {
                            nps += effect_nps;
                        }
                    }
                },
            }
        }
    }
    nps
}
