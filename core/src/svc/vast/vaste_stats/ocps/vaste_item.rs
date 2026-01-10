use crate::{
    num::PValue,
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatTimeOptions, Vast, vaste_stats::item_checks::check_drone_fighter_module},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_cps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<PValue, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_uid)?;
        let mut ocps = PValue::ZERO;
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, ignore_state) {
            Some(cseq_map) => cseq_map,
            None => return Ok(ocps),
        };
        for (effect_rid, cseq) in cseq_map {
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let ospec = match effect.outgoing_cap_opc_spec {
                Some(ospec) => ospec,
                None => continue,
            };
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_ocps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        &cseq,
                        &ospec,
                        projectee_uid,
                        burst_opts.spool,
                    ) {
                        ocps += effect_ocps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if let Some(effect_ocps) =
                            aggr_proj_time_ps(ctx, calc, item_uid, effect, &cseq, &ospec, projectee_uid, time)
                        {
                            ocps += effect_ocps;
                        }
                    }
                    _ => {
                        if let Some(effect_ocps) =
                            aggr_proj_looped_ps(ctx, calc, item_uid, effect, &cseq, &ospec, projectee_uid)
                        {
                            ocps += effect_ocps;
                        }
                    }
                },
            }
        }
        Ok(ocps)
    }
}
