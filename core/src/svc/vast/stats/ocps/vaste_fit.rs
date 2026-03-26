use crate::{
    nd::NGeneralOutputGetter,
    num::PValue,
    rd::{REffectId, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        vast::{
            StatOutRepItemKinds, StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_first, aggr_proj_looped, aggr_proj_time},
        },
    },
    ud::{UFitId, UItemId},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_outgoing_cps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> PValue {
        fit_uids
            .map(|fit_uid| {
                get_ocps(
                    ctx,
                    calc,
                    StatOutRepItemKinds::all_enabled(),
                    time_options,
                    projectee_uid,
                    &self.get_fit_data(&fit_uid).out_cap,
                )
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_cps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> PValue {
        let fit_data = self.get_fit_data(&fit_uid);
        get_ocps(
            ctx,
            calc,
            StatOutRepItemKinds::all_enabled(),
            time_options,
            projectee_uid,
            &fit_data.out_cap,
        )
    }
}

fn get_ocps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatOutRepItemKinds,
    time_options: StatTimeOptions,
    projectee_uid: Option<UItemId>,
    fit_data: &RMapRMap<UItemId, REffectId, REffectProjOpcSpec<NGeneralOutputGetter>>,
) -> PValue {
    let mut orps = PValue::ZERO;
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
                    (),
                    projectee_uid,
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
                        projectee_uid,
                        &mut accum,
                        time,
                    ),
                    _ => aggr_proj_looped(ctx, calc, item_uid, effect, cseq, ospec, (), projectee_uid, &mut accum),
                },
            } {
                orps += accum.get_per_second();
            }
        }
    }
    orps
}
