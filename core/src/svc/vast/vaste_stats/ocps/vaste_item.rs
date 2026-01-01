use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::get_item_cseq_map,
        err::StatItemCheckError,
        vast::{StatTimeOptions, Vast, vaste_stats::item_checks::check_drone_fighter_module},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_cps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_key)?;
        let mut ocps = OF(0.0);
        let cycling_options = time_options.into();
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, ignore_state) {
            Some(cseq_map) => cseq_map,
            None => return Ok(ocps),
        };
        for (effect_key, cseq) in cseq_map {
            let effect = ctx.u_data.src.get_effect(effect_key);
            let ospec = match effect.outgoing_cap_opc_spec {
                Some(ospec) => ospec,
                None => continue,
            };
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_ocps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        item_key,
                        effect,
                        &cseq,
                        &ospec,
                        projectee_key,
                        burst_opts.spool,
                    ) {
                        ocps += effect_ocps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_ocps) =
                            aggr_proj_time_ps(ctx, calc, item_key, effect, &cseq, &ospec, projectee_key, time)
                        {
                            ocps += effect_ocps;
                        }
                    }
                    _ => {
                        if let Some(effect_ocps) =
                            aggr_proj_looped_ps(ctx, calc, item_key, effect, &cseq, &ospec, projectee_key)
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
