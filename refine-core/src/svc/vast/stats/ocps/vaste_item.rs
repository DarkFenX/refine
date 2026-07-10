use crate::{
    num::PValue,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_burst, aggr_proj_looped, aggr_proj_time},
            stats::item_checks::check_drone_fighter_module,
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_cps(
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> Result<PValue, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_uid)?;
        let mut ocps = PValue::ZERO;
        let cycling_options = CyclingOptions::from_time_options(time_options);
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
            return Ok(ocps);
        }
        for (&effect_rid, cseq) in reuse_cseq_map.iter() {
            let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
            let Some(ospec) = effect.outgoing_cap else {
                continue;
            };
            if let Some(accum) = match time_options {
                StatTimeOptions::Burst(burst_opts) => aggr_proj_burst(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    &ospec,
                    (),
                    projectee_uid,
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
                        &ospec,
                        (),
                        projectee_uid,
                        SeqAccum::new_stack(),
                        time,
                    ),
                    _ => aggr_proj_looped(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
                        &ospec,
                        (),
                        projectee_uid,
                        SeqAccum::new_stack(),
                    ),
                },
            } {
                ocps += accum.get_per_second();
            }
        }
        Ok(ocps)
    }
}
