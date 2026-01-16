use std::collections::BinaryHeap;

use super::{
    aggregate::Aggregator,
    event::{CapSimEvent, CapSimEventInjector},
    stagger::{StaggerKey, StatCapSimStaggerInt, process_staggers},
};
use crate::{
    num::PValue,
    svc::{
        SvcCtx,
        aggr::{AggrLocalInvData, AggrProjInvData, get_local_output, get_proj_output},
        calc::Calc,
        cycle::{CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        output::{Output, OutputSimple},
        vast::{Vast, VastFitData},
    },
    ud::UItemId,
    util::RMapVec,
};

pub(super) fn prepare_events(
    ctx: SvcCtx,
    calc: &mut Calc,
    vast: &Vast,
    reload_optionals: Option<bool>,
    stagger: StatCapSimStaggerInt,
    fit_data: &VastFitData,
    cap_item_uid: UItemId,
) -> BinaryHeap<CapSimEvent> {
    let cycling_options = CyclingOptions::Sim(CycleOptionsSim { reload_optionals, .. });
    let mut aggregator = Aggregator::new();
    fill_consumers(ctx, calc, &mut aggregator, cycling_options, &stagger, fit_data);
    fill_nosfs(ctx, calc, &mut aggregator, cycling_options, &stagger, fit_data);
    fill_incoming_neuts(
        ctx,
        calc,
        &mut aggregator,
        cycling_options,
        &stagger,
        vast,
        cap_item_uid,
    );
    fill_incoming_transfers(
        ctx,
        calc,
        &mut aggregator,
        cycling_options,
        &stagger,
        vast,
        cap_item_uid,
    );
    let mut events = BinaryHeap::new();
    aggregator.into_sim_events(&mut events);
    fill_injectors(ctx, calc, &mut events, cycling_options, fit_data);
    events
}

fn fill_consumers(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    cycling_options: CyclingOptions,
    stagger: &StatCapSimStaggerInt,
    fit_data: &VastFitData,
) {
    let mut stagger_map = RMapVec::new();
    for (&item_uid, item_data) in fit_data.cap_consumers.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, &attr_rid) in item_data.iter() {
            let cap_consumed = match calc.get_item_attr_oextra(ctx, item_uid, attr_rid) {
                // Cap consumed can be negative value, e.g. for nosfs
                Some(cap_consumed) if cap_consumed.abs() > PValue::FLOAT_TOLERANCE => cap_consumed,
                _ => continue,
            };
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let opc = Output::Simple(OutputSimple {
                amount: -cap_consumed,
                delay: PValue::ZERO,
            });
            match stagger.is_staggered(item_uid) {
                true => stagger_map.add_entry(StaggerKey::new(&cseq.convert(), &opc), (cseq.convert(), opc)),
                false => aggregator.add_entry(PValue::ZERO, cseq.convert(), opc),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_nosfs(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    cycling_options: CyclingOptions,
    stagger: &StatCapSimStaggerInt,
    fit_data: &VastFitData,
) {
    let mut stagger_map = RMapVec::new();
    for (&nosf_item_uid, item_data) in fit_data.cap_nosfs.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, nosf_item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let inv_proj = match AggrProjInvData::try_make(ctx, calc, nosf_item_uid, effect, ospec, None) {
                Some(inv_proj) => inv_proj,
                None => continue,
            };
            let opc = get_proj_output(ctx, calc, nosf_item_uid, ospec, &inv_proj, None);
            if !opc.has_impact() {
                continue;
            }
            let opc = opc.into_value();
            match stagger.is_staggered(nosf_item_uid) {
                true => stagger_map.add_entry(StaggerKey::new(&cseq.convert(), &opc), (cseq.convert(), opc)),
                false => aggregator.add_entry(PValue::ZERO, cseq.convert(), opc),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_incoming_neuts(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    cycling_options: CyclingOptions,
    stagger: &StatCapSimStaggerInt,
    vast: &Vast,
    cap_item_uid: UItemId,
) {
    let neut_data = match vast.in_neuts.get_l1(&cap_item_uid) {
        Some(neut_data) => neut_data,
        None => return,
    };
    let mut stagger_map = RMapVec::new();
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
            let inv_proj = match AggrProjInvData::try_make(ctx, calc, neut_item_uid, effect, ospec, Some(cap_item_uid))
            {
                Some(inv_proj) => inv_proj,
                None => continue,
            };
            let opc = get_proj_output(ctx, calc, neut_item_uid, ospec, &inv_proj, None);
            if !opc.has_impact() {
                continue;
            }
            // Negate output, since neuts negatively impact cap, but output of neut getter function
            // is positive
            let opc = -opc;
            match stagger.is_staggered(neut_item_uid) {
                true => stagger_map.add_entry(StaggerKey::new(&cseq.convert(), &opc), (cseq.convert(), opc)),
                false => aggregator.add_entry(PValue::ZERO, cseq.convert(), opc),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_incoming_transfers(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    cycling_options: CyclingOptions,
    stagger: &StatCapSimStaggerInt,
    vast: &Vast,
    cap_item_uid: UItemId,
) {
    let transfer_data = match vast.in_cap.get_l1(&cap_item_uid) {
        Some(transfer_data) => transfer_data,
        None => return,
    };
    let mut stagger_map = RMapVec::new();
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
            let inv_proj =
                match AggrProjInvData::try_make(ctx, calc, transfer_item_uid, effect, ospec, Some(cap_item_uid)) {
                    Some(inv_proj) => inv_proj,
                    None => continue,
                };
            let opc = get_proj_output(ctx, calc, transfer_item_uid, ospec, &inv_proj, None);
            if !opc.has_impact() {
                continue;
            }
            let opc = opc.into_value();
            match stagger.is_staggered(transfer_item_uid) {
                true => stagger_map.add_entry(StaggerKey::new(&cseq.convert(), &opc), (cseq.convert(), opc)),
                false => aggregator.add_entry(PValue::ZERO, cseq.convert(), opc),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_injectors(
    ctx: SvcCtx,
    calc: &mut Calc,
    events: &mut BinaryHeap<CapSimEvent>,
    cycling_options: CyclingOptions,
    fit_data: &VastFitData,
) {
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
            let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec) {
                Some(inv_local) => inv_local,
                None => continue,
            };
            let opc = get_local_output(ctx, calc, item_uid, ospec, &inv_local, None);
            let immediate_amount = opc
                .iter_amounts()
                .next()
                .and_then(|v| match v.time_passed == PValue::ZERO {
                    true => Some(v.amount),
                    false => None,
                });
            events.push(CapSimEvent::InjectorReady(CapSimEventInjector {
                time: PValue::ZERO,
                cycle_iter: cseq.convert().iter_cycles(),
                opc: opc.into_value(),
                immediate_amount: immediate_amount.map(|v| v.into_value()),
            }));
        }
    }
}
