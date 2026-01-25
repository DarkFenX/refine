use crate::{
    num::PValue,
    rd::{REffectId, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        vast::{
            StatNeutItemKinds, StatTimeOptions, Vast,
            aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        },
    },
    ud::{UFitId, UItemId},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_outgoing_nps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> PValue {
        fit_uids
            .map(|fit_uid| {
                get_nps(
                    ctx,
                    calc,
                    item_kinds,
                    time_options,
                    projectee_uid,
                    &self.get_fit_data(&fit_uid).out_neuts,
                )
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_nps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> PValue {
        let fit_data = self.get_fit_data(&fit_uid);
        get_nps(ctx, calc, item_kinds, time_options, projectee_uid, &fit_data.out_neuts)
    }
}

fn get_nps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatNeutItemKinds,
    time_options: StatTimeOptions,
    projectee_item_uid: Option<UItemId>,
    fit_data: &RMapRMap<UItemId, REffectId, REffectProjOpcSpec<PValue>>,
) -> PValue {
    let mut nps = PValue::ZERO;
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
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_nps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
                        ospec,
                        projectee_item_uid,
                        burst_opts.spool,
                    ) {
                        nps += effect_nps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if let Some(effect_nps) =
                            aggr_proj_time_ps(ctx, calc, item_uid, effect, cseq, ospec, projectee_item_uid, time)
                        {
                            nps += effect_nps;
                        }
                    }
                    _ => {
                        if let Some(effect_nps) =
                            aggr_proj_looped_ps(ctx, calc, item_uid, effect, cseq, ospec, projectee_item_uid)
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
