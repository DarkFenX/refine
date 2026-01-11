use std::cmp::Ordering;

use super::option::StatCapSrcKinds;
use crate::{
    num::{PValue, UnitInterval, Value},
    svc::{
        SvcCtx,
        aggr::{
            aggr_local_first_ps, aggr_local_looped_ps, aggr_local_time_ps, aggr_proj_first_ps, aggr_proj_looped_ps,
            aggr_proj_time_ps,
        },
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatTimeOptions, Vast, VastFitData, shared::calc_regen, vaste_stats::item_checks::check_ship},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_balance(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        src_kinds: StatCapSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<Value, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_uid)?;
        let fit_data = self.fit_datas.get(&ship.get_fit_uid()).unwrap();
        let mut balance = Value::ZERO;
        if src_kinds.regen.enabled {
            balance += get_cap_regen(ctx, calc, item_uid, src_kinds.regen.cap_perc);
        }
        if src_kinds.cap_injectors {
            balance += get_cap_injects(ctx, calc, time_options, fit_data);
        }
        if src_kinds.consumers || src_kinds.nosfs {
            balance -= get_cap_consumed(ctx, calc, time_options, fit_data, src_kinds.consumers, src_kinds.nosfs);
        }
        if src_kinds.incoming_transfers {
            balance += get_cap_transfers(ctx, calc, time_options, item_uid, self);
        }
        if src_kinds.incoming_neuts {
            balance -= get_neuts(ctx, calc, time_options, item_uid, self);
        }
        Ok(balance)
    }
}

fn get_cap_regen(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, cap_perc: UnitInterval) -> PValue {
    let max_amount = Vast::internal_get_stat_item_cap_amount_unchecked(ctx, calc, item_uid);
    let cap_regen_time = Vast::internal_get_stat_item_cap_recharge_time_unchecked(ctx, calc, item_uid);
    calc_regen(max_amount, cap_regen_time, cap_perc)
}

fn get_cap_injects(ctx: SvcCtx, calc: &mut Calc, time_options: StatTimeOptions, fit_data: &VastFitData) -> PValue {
    let mut cps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in fit_data.cap_injects.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            match time_options {
                StatTimeOptions::Burst(_) => {
                    if let Some(effect_cps) = aggr_local_first_ps(ctx, calc, item_uid, effect, cseq, ospec) {
                        cps += effect_cps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if let Some(effect_cps) = aggr_local_time_ps(ctx, calc, item_uid, effect, cseq, ospec, time) {
                            cps += effect_cps;
                        }
                    }
                    _ => {
                        if let Some(effect_cps) = aggr_local_looped_ps(ctx, calc, item_uid, effect, cseq, ospec) {
                            cps += effect_cps;
                        }
                    }
                },
            }
        }
    }
    cps
}

fn get_cap_consumed(
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    fit_data: &VastFitData,
    drains: bool,
    gains: bool,
) -> Value {
    let mut cps = Value::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in fit_data.cap_consumers_active.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, &attr_rid) in item_data.iter() {
            let cap_consumed = match calc.get_item_attr_oextra(ctx, item_uid, attr_rid) {
                Some(cap_consumed) => cap_consumed,
                None => continue,
            };
            match (cap_consumed.cmp(&Value::ZERO), drains, gains) {
                (Ordering::Greater, true, _) | (Ordering::Less, _, true) => (),
                _ => continue,
            };
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            cps += cap_consumed / cseq.get_average_time();
        }
    }
    cps
}

fn get_cap_transfers(
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    cap_item_uid: UItemId,
    vast: &Vast,
) -> PValue {
    let mut cps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    let transfer_data = match vast.in_cap.get_l1(&cap_item_uid) {
        Some(transfer_data) => transfer_data,
        None => return cps,
    };
    for (&transfer_item_uid, item_data) in transfer_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, transfer_item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_cps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        transfer_item_uid,
                        effect,
                        cseq,
                        ospec,
                        Some(cap_item_uid),
                        burst_opts.spool,
                    ) {
                        cps += effect_cps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if let Some(effect_cps) = aggr_proj_time_ps(
                            ctx,
                            calc,
                            transfer_item_uid,
                            effect,
                            cseq,
                            ospec,
                            Some(cap_item_uid),
                            time,
                        ) {
                            cps += effect_cps;
                        }
                    }
                    _ => {
                        if let Some(effect_cps) =
                            aggr_proj_looped_ps(ctx, calc, transfer_item_uid, effect, cseq, ospec, Some(cap_item_uid))
                        {
                            cps += effect_cps;
                        }
                    }
                },
            }
        }
    }
    cps
}

fn get_neuts(
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    cap_item_uid: UItemId,
    vast: &Vast,
) -> PValue {
    let mut nps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    let neut_data = match vast.in_neuts.get_l1(&cap_item_uid) {
        Some(neut_data) => neut_data,
        None => return nps,
    };
    for (&neut_item_uid, item_data) in neut_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, neut_item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
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
                        neut_item_uid,
                        effect,
                        cseq,
                        ospec,
                        Some(cap_item_uid),
                        burst_opts.spool,
                    ) {
                        nps += effect_nps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if let Some(effect_nps) =
                            aggr_proj_time_ps(ctx, calc, neut_item_uid, effect, cseq, ospec, Some(cap_item_uid), time)
                        {
                            nps += effect_nps;
                        }
                    }
                    _ => {
                        if let Some(effect_nps) =
                            aggr_proj_looped_ps(ctx, calc, neut_item_uid, effect, cseq, ospec, Some(cap_item_uid))
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
