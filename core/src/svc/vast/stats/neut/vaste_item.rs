use crate::{
    num::PValue,
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatTimeOptions, Vast, stats::item_checks::check_charge_drone_fighter_module},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_nps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<PValue, StatItemCheckError> {
        check_charge_drone_fighter_module(ctx.u_data, item_uid)?;
        let mut nps = PValue::ZERO;
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, ignore_state) {
            Some(cseq_map) => cseq_map,
            None => return Ok(nps),
        };
        for (effect_rid, cseq) in cseq_map {
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let ospec = match effect.neut_opc_spec {
                Some(ospec) => ospec,
                None => continue,
            };
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_nps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        &cseq,
                        &ospec,
                        projectee_uid,
                        burst_opts.spool,
                    ) {
                        nps += effect_nps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if let Some(effect_nps) =
                            aggr_proj_time_ps(ctx, calc, item_uid, effect, &cseq, &ospec, projectee_uid, time)
                        {
                            nps += effect_nps;
                        }
                    }
                    _ => {
                        if let Some(effect_nps) =
                            aggr_proj_looped_ps(ctx, calc, item_uid, effect, &cseq, &ospec, projectee_uid)
                        {
                            nps += effect_nps;
                        }
                    }
                },
            }
        }
        if include_charges {
            for charge_uid in ctx.u_data.items.get(item_uid).iter_charges() {
                if let Ok(charge_nps) = Vast::get_stat_item_outgoing_nps(
                    ctx,
                    calc,
                    charge_uid,
                    time_options,
                    false,
                    ignore_state,
                    projectee_uid,
                ) {
                    nps += charge_nps;
                }
            }
        }
        Ok(nps)
    }
}
