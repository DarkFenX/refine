use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::get_item_cseq_map,
        err::StatItemCheckError,
        vast::{StatTimeOptions, Vast, vaste_stats::item_checks::check_charge_drone_fighter_module},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_nps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_charge_drone_fighter_module(ctx.u_data, item_key)?;
        let mut nps = OF(0.0);
        let cycling_options = time_options.into();
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, ignore_state) {
            Some(cseq_map) => cseq_map,
            None => return Ok(nps),
        };
        for (effect_key, cseq) in cseq_map {
            let effect = ctx.u_data.src.get_effect(effect_key);
            let ospec = match effect.neut_opc_spec {
                Some(ospec) => ospec,
                None => continue,
            };
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_nps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        item_key,
                        effect,
                        &cseq,
                        &ospec,
                        projectee_key,
                        burst_opts.spool,
                    ) {
                        nps += effect_nps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_nps) =
                            aggr_proj_time_ps(ctx, calc, item_key, effect, &cseq, &ospec, projectee_key, time)
                        {
                            nps += effect_nps;
                        }
                    }
                    _ => {
                        if let Some(effect_nps) =
                            aggr_proj_looped_ps(ctx, calc, item_key, effect, &cseq, &ospec, projectee_key)
                        {
                            nps += effect_nps;
                        }
                    }
                },
            }
        }
        if include_charges {
            for charge_key in ctx.u_data.items.get(item_key).iter_charges() {
                if let Ok(charge_nps) = Vast::get_stat_item_outgoing_nps(
                    ctx,
                    calc,
                    charge_key,
                    time_options,
                    false,
                    ignore_state,
                    projectee_key,
                ) {
                    nps += charge_nps;
                }
            }
        }
        Ok(nps)
    }
}
