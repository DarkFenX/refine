use crate::{
    nd::NEffectGeneralOutputGetter,
    num::PValue,
    rd::{REffectId, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        vast::{
            StatNeutItemKinds, StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_burst, aggr_proj_looped, aggr_proj_time},
        },
    },
    ud::{UFitId, UItemId},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_outgoing_nps(
        &self,
        reuse_cseq_map: &mut CseqMap,
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
                    reuse_cseq_map,
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
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> PValue {
        let fit_data = self.get_fit_data(&fit_uid);
        get_nps(
            reuse_cseq_map,
            ctx,
            calc,
            item_kinds,
            time_options,
            projectee_uid,
            &fit_data.out_neuts,
        )
    }
}

fn get_nps(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatNeutItemKinds,
    time_options: StatTimeOptions,
    projectee_item_uid: Option<UItemId>,
    fit_data: &RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
) -> PValue {
    let mut nps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in fit_data.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
            continue;
        }
        for (&effect_rid, ospec) in item_data.iter() {
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if !item_kinds.resolve(effect) {
                continue;
            }
            let cseq = match reuse_cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let mut accum = SeqAccum::new_stack();
            if match time_options {
                StatTimeOptions::Burst(burst_opts) => aggr_proj_burst(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    ospec,
                    (),
                    projectee_item_uid,
                    burst_opts.spool,
                    &mut accum,
                ),
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => aggr_proj_time(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        projectee_item_uid,
                        &mut accum,
                        time,
                    ),
                    _ => aggr_proj_looped(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        projectee_item_uid,
                        &mut accum,
                    ),
                },
            } {
                nps += accum.get_per_second();
            }
        }
    }
    nps
}
