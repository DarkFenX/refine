use super::option::{StatCapBlcNosfsInt, StatCapBlcRegen, StatCapBlcSrcKindsInt};
use crate::{
    num::{PValue, UnitInterval, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatTimeOptions, Vast, VastFitData,
            aggr::{
                SeqAccum, aggr_local_burst, aggr_local_looped, aggr_local_time, aggr_proj_burst, aggr_proj_looped,
                aggr_proj_time,
            },
            stats::{item_checks::check_ship, shared::calc_regen_for_attrs},
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_balance(
        &self,
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        src_kinds: StatCapBlcSrcKindsInt,
        time_options: StatTimeOptions,
    ) -> Result<Value, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_uid)?;
        let fit_data = self.fit_datas.get(ship.get_fit_uid());
        let mut balance = Value::ZERO;
        if let StatCapBlcRegen::Enabled(regen_options) = src_kinds.regen {
            balance += get_cap_regen(ctx, calc, item_uid, regen_options.cap_perc);
        }
        if src_kinds.cap_injectors {
            balance += get_cap_injects(reuse_cseq_map, ctx, calc, time_options, fit_data);
        }
        if let StatCapBlcNosfsInt::Enabled(nosfs_options) = src_kinds.nosfs {
            balance += get_nosfs(
                reuse_cseq_map,
                ctx,
                calc,
                time_options,
                nosfs_options.projectee_item_uid,
                fit_data,
            );
        }
        if src_kinds.consumers {
            balance -= get_cap_consumed(reuse_cseq_map, ctx, calc, time_options, fit_data);
        }
        if src_kinds.incoming_transfers {
            balance += get_incoming_cap_transfers(reuse_cseq_map, ctx, calc, time_options, item_uid, self);
        }
        if src_kinds.incoming_neuts {
            balance -= get_incoming_neuts(reuse_cseq_map, ctx, calc, time_options, item_uid, self);
        }
        Ok(balance)
    }
}

fn get_cap_regen(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, cap_perc: UnitInterval) -> PValue {
    calc_regen_for_attrs(
        ctx,
        calc,
        item_uid,
        ctx.ac().capacitor_capacity,
        ctx.ac().recharge_rate,
        cap_perc,
    )
}

fn get_cap_injects(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    fit_data: &VastFitData,
) -> PValue {
    let mut cps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in fit_data.cap_injects.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
            continue;
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let Some(cseq) = reuse_cseq_map.get(&effect_rid) else {
                continue;
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(accum) = match time_options {
                StatTimeOptions::Burst(_) => {
                    aggr_local_burst(ctx, calc, item_uid, effect, cseq, ospec, (), SeqAccum::new_stack())
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => aggr_local_time(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        SeqAccum::new_stack(),
                        time,
                    ),
                    _ => aggr_local_looped(ctx, calc, item_uid, effect, cseq, ospec, (), SeqAccum::new_stack()),
                },
            } {
                cps += accum.get_per_second();
            }
        }
    }
    cps
}

fn get_cap_consumed(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    fit_data: &VastFitData,
) -> Value {
    let mut cps = Value::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in fit_data.cap_consumers.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
            continue;
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let Some(cseq) = reuse_cseq_map.get(&effect_rid) else {
                continue;
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(accum) = match time_options {
                StatTimeOptions::Burst(_) => {
                    aggr_local_burst(ctx, calc, item_uid, effect, cseq, ospec, (), SeqAccum::new_stack())
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => aggr_local_time(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        SeqAccum::new_stack(),
                        time,
                    ),
                    _ => aggr_local_looped(ctx, calc, item_uid, effect, cseq, ospec, (), SeqAccum::new_stack()),
                },
            } {
                cps += accum.get_per_second();
            }
        }
    }
    cps
}

fn get_nosfs(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    projectee_item_uid: Option<UItemId>,
    fit_data: &VastFitData,
) -> Value {
    let mut cps = Value::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&nosf_item_uid, item_data) in fit_data.cap_nosfs.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, nosf_item_uid, cycling_options) {
            continue;
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let Some(cseq) = reuse_cseq_map.get(&effect_rid) else {
                continue;
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(accum) = match time_options {
                StatTimeOptions::Burst(burst_opts) => aggr_proj_burst(
                    ctx,
                    calc,
                    nosf_item_uid,
                    effect,
                    cseq,
                    ospec,
                    (),
                    projectee_item_uid,
                    burst_opts.spool,
                    SeqAccum::new_stack(),
                ),
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => aggr_proj_time(
                        ctx,
                        calc,
                        nosf_item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        projectee_item_uid,
                        SeqAccum::new_stack(),
                        time,
                    ),
                    _ => aggr_proj_looped(
                        ctx,
                        calc,
                        nosf_item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        projectee_item_uid,
                        SeqAccum::new_stack(),
                    ),
                },
            } {
                cps += accum.get_per_second();
            }
        }
    }
    cps
}

fn get_incoming_cap_transfers(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    cap_item_uid: UItemId,
    vast: &Vast,
) -> PValue {
    let mut cps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    let Some(transfer_data) = vast.in_cap.get_l1(&cap_item_uid) else {
        return cps;
    };
    for (&transfer_item_uid, item_data) in transfer_data.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, transfer_item_uid, cycling_options) {
            continue;
        }
        for (&effect_rid, ospec) in item_data.iter() {
            let Some(cseq) = reuse_cseq_map.get(&effect_rid) else {
                continue;
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(accum) = match time_options {
                StatTimeOptions::Burst(burst_opts) => aggr_proj_burst(
                    ctx,
                    calc,
                    transfer_item_uid,
                    effect,
                    cseq,
                    ospec,
                    (),
                    Some(cap_item_uid),
                    burst_opts.spool,
                    SeqAccum::new_stack(),
                ),
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => aggr_proj_time(
                        ctx,
                        calc,
                        transfer_item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        Some(cap_item_uid),
                        SeqAccum::new_stack(),
                        time,
                    ),
                    _ => aggr_proj_looped(
                        ctx,
                        calc,
                        transfer_item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        Some(cap_item_uid),
                        SeqAccum::new_stack(),
                    ),
                },
            } {
                cps += accum.get_per_second();
            }
        }
    }
    cps
}

fn get_incoming_neuts(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    cap_item_uid: UItemId,
    vast: &Vast,
) -> PValue {
    let mut nps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    let Some(neut_data) = vast.in_neuts.get_l1(&cap_item_uid) else {
        return nps;
    };
    for (&neut_item_uid, item_data) in neut_data.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, neut_item_uid, cycling_options) {
            continue;
        }
        for (&effect_rid, ospec) in item_data.iter() {
            let Some(cseq) = reuse_cseq_map.get(&effect_rid) else {
                continue;
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(accum) = match time_options {
                StatTimeOptions::Burst(burst_opts) => aggr_proj_burst(
                    ctx,
                    calc,
                    neut_item_uid,
                    effect,
                    cseq,
                    ospec,
                    (),
                    Some(cap_item_uid),
                    burst_opts.spool,
                    SeqAccum::new_stack(),
                ),
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => aggr_proj_time(
                        ctx,
                        calc,
                        neut_item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        Some(cap_item_uid),
                        SeqAccum::new_stack(),
                        time,
                    ),
                    _ => aggr_proj_looped(
                        ctx,
                        calc,
                        neut_item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        Some(cap_item_uid),
                        SeqAccum::new_stack(),
                    ),
                },
            } {
                nps += accum.get_per_second();
            }
        }
    }
    nps
}
