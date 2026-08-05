use crate::{
    PValue,
    stats::{StatChargeOptions, StatCritOptions, StatTimeOptions},
    svc::{
        Calc, SvcCtx, Vast,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        err::IntStatItemError,
        vast::{
            aggr::{SeqAccum, aggr_proj_burst, aggr_proj_looped, aggr_proj_time},
            stats::item_checks::check_charge_drone_fighter_module,
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_nps(
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        charge_options: StatChargeOptions,
        projectee_uid: Option<UItemId>,
    ) -> Result<PValue, IntStatItemError<!>> {
        check_charge_drone_fighter_module(ctx.u_data, item_uid)?;
        let mut nps = PValue::ZERO;
        let cycling_options = CyclingOptions::from_time_options(time_options);
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
            return Ok(nps);
        }
        let u_item = ctx.u_data.items.get(item_uid);
        for (&effect_rid, cseq) in reuse_cseq_map.iter() {
            let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
            let ospec = match &effect.neut {
                Some(neut) if neut.check(u_item, ctx.ac()) => neut.ospec,
                _ => continue,
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
                    StatCritOptions::default(),
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
                        StatCritOptions::default(),
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
                        StatCritOptions::default(),
                        projectee_uid,
                        SeqAccum::new_stack(),
                    ),
                },
            } {
                nps += accum.get_per_second();
            }
        }
        if charge_options.is_enabled() {
            for charge_uid in u_item.iter_charges() {
                if let Ok(charge_nps) = Self::get_stat_item_outgoing_nps(
                    reuse_cseq_map,
                    ctx,
                    calc,
                    charge_uid,
                    time_options,
                    StatChargeOptions::Exclude,
                    projectee_uid,
                ) {
                    nps += charge_nps;
                }
            }
        }
        Ok(nps)
    }
}
