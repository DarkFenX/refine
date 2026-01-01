use crate::{
    def::{AttrVal, OF},
    rd::{REffectKey, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::get_item_cseq_map,
        vast::{StatOutRepItemKinds, StatTimeOptions, Vast},
    },
    ud::{UFitKey, UItemKey},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_outgoing_cps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemKey>,
    ) -> AttrVal {
        fit_keys
            .map(|fit_key| {
                get_ocps(
                    ctx,
                    calc,
                    StatOutRepItemKinds::all_enabled(),
                    time_options,
                    projectee_key,
                    &self.get_fit_data(&fit_key).out_cap,
                )
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_cps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemKey>,
    ) -> AttrVal {
        let fit_data = self.get_fit_data(&fit_key);
        get_ocps(
            ctx,
            calc,
            StatOutRepItemKinds::all_enabled(),
            time_options,
            projectee_key,
            &fit_data.out_cap,
        )
    }
}

fn get_ocps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatOutRepItemKinds,
    time_options: StatTimeOptions,
    projectee_key: Option<UItemKey>,
    fit_data: &RMapRMap<UItemKey, REffectKey, REffectProjOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut orps = OF(0.0);
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
                    if let Some(effect_orps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        item_key,
                        effect,
                        cseq,
                        ospec,
                        projectee_key,
                        burst_opts.spool,
                    ) {
                        orps += effect_orps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_orps) =
                            aggr_proj_time_ps(ctx, calc, item_key, effect, cseq, ospec, projectee_key, time)
                        {
                            orps += effect_orps;
                        }
                    }
                    _ => {
                        if let Some(effect_orps) =
                            aggr_proj_looped_ps(ctx, calc, item_key, effect, cseq, ospec, projectee_key)
                        {
                            orps += effect_orps;
                        }
                    }
                },
            }
        }
    }
    orps
}
