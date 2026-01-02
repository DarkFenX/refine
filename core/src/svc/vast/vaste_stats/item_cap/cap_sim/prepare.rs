use std::collections::BinaryHeap;

use ordered_float::Float;

use super::{
    aggregate::Aggregator,
    event::{CapSimEvent, CapSimEventInjector},
    stagger::{StaggerKey, StatCapSimStaggerInt, process_staggers},
};
use crate::{
    def::OF,
    svc::{
        SvcCtx,
        aggr::{AggrLocalInvData, AggrProjInvData, get_local_output, get_proj_output},
        calc::Calc,
        cycle::{CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        output::{Output, OutputSimple},
        vast::{Vast, VastFitData},
    },
    ud::UItemId,
    util::{FLOAT_TOLERANCE, RMapVec},
};

pub(super) fn prepare_events(
    ctx: SvcCtx,
    calc: &mut Calc,
    vast: &Vast,
    reload_optionals: Option<bool>,
    stagger: StatCapSimStaggerInt,
    fit_data: &VastFitData,
    cap_item_key: UItemId,
) -> BinaryHeap<CapSimEvent> {
    let mut aggregator = Aggregator::new();
    fill_consumers(ctx, calc, &mut aggregator, reload_optionals, &stagger, fit_data);
    fill_neuts(
        ctx,
        calc,
        &mut aggregator,
        reload_optionals,
        &stagger,
        vast,
        cap_item_key,
    );
    fill_transfers(
        ctx,
        calc,
        &mut aggregator,
        reload_optionals,
        &stagger,
        vast,
        cap_item_key,
    );
    let mut events = BinaryHeap::new();
    aggregator.into_sim_events(&mut events);
    fill_injectors(ctx, calc, &mut events, reload_optionals, fit_data);
    events
}

fn get_cycling_options(reload_optionals: Option<bool>) -> CyclingOptions {
    CyclingOptions::Sim(CycleOptionsSim { reload_optionals, .. })
}

fn fill_consumers(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    reload_optionals: Option<bool>,
    stagger: &StatCapSimStaggerInt,
    fit_data: &VastFitData,
) {
    let mut stagger_map = RMapVec::new();
    let cycling_options = get_cycling_options(reload_optionals);
    for (&item_key, item_data) in fit_data.cap_consumers_active.iter() {
        let cycle_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, &attr_key) in item_data.iter() {
            let cap_consumed = match calc.get_item_attr_oextra(ctx, item_key, attr_key) {
                // Cap consumed can be negative value, e.g. for nosfs
                Some(cap_consumed) if cap_consumed.abs() > FLOAT_TOLERANCE => cap_consumed,
                _ => continue,
            };
            let effect_cycles = match cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let output_per_cycle = Output::Simple(OutputSimple {
                amount: -cap_consumed,
                delay: OF(0.0),
            });
            match stagger.is_staggered(item_key) {
                true => stagger_map.add_entry(
                    StaggerKey::new(&effect_cycles.convert(), &output_per_cycle),
                    (effect_cycles.convert(), output_per_cycle),
                ),
                false => aggregator.add_entry(OF(0.0), effect_cycles.convert(), output_per_cycle),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_neuts(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    reload_optionals: Option<bool>,
    stagger: &StatCapSimStaggerInt,
    vast: &Vast,
    cap_item_key: UItemId,
) {
    let neut_data = match vast.in_neuts.get_l1(&cap_item_key) {
        Some(neut_data) => neut_data,
        None => return,
    };
    let mut stagger_map = RMapVec::new();
    let cycling_options = get_cycling_options(reload_optionals);
    for (&neut_item_key, item_data) in neut_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, neut_item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            let inv_proj = match AggrProjInvData::try_make(ctx, calc, neut_item_key, effect, ospec, Some(cap_item_key))
            {
                Some(inv_proj) => inv_proj,
                None => continue,
            };
            // Negate output, since neuts negatively impact cap, but output of neut getter
            // function is positive
            let opc = -get_proj_output(ctx, calc, neut_item_key, ospec, &inv_proj, None);
            if !opc.has_impact() {
                continue;
            }
            match stagger.is_staggered(neut_item_key) {
                true => stagger_map.add_entry(StaggerKey::new(&cseq.convert(), &opc), (cseq.convert(), opc)),
                false => aggregator.add_entry(OF(0.0), cseq.convert(), opc),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_transfers(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    reload_optionals: Option<bool>,
    stagger: &StatCapSimStaggerInt,
    vast: &Vast,
    cap_item_key: UItemId,
) {
    let transfer_data = match vast.in_cap.get_l1(&cap_item_key) {
        Some(transfer_data) => transfer_data,
        None => return,
    };
    let mut stagger_map = RMapVec::new();
    let cycling_options = get_cycling_options(reload_optionals);
    for (&transfer_item_key, item_data) in transfer_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, transfer_item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            let inv_proj =
                match AggrProjInvData::try_make(ctx, calc, transfer_item_key, effect, ospec, Some(cap_item_key)) {
                    Some(inv_proj) => inv_proj,
                    None => continue,
                };
            let opc = get_proj_output(ctx, calc, transfer_item_key, ospec, &inv_proj, None);
            if !opc.has_impact() {
                continue;
            }
            match stagger.is_staggered(transfer_item_key) {
                true => stagger_map.add_entry(StaggerKey::new(&cseq.convert(), &opc), (cseq.convert(), opc)),
                false => aggregator.add_entry(OF(0.0), cseq.convert(), opc),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_injectors(
    ctx: SvcCtx,
    calc: &mut Calc,
    events: &mut BinaryHeap<CapSimEvent>,
    reload_optionals: Option<bool>,
    fit_data: &VastFitData,
) {
    let cycling_options = get_cycling_options(reload_optionals);
    for (&item_key, item_data) in fit_data.cap_injects.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_key, effect, ospec) {
                Some(inv_local) => inv_local,
                None => continue,
            };
            let opc = get_local_output(ctx, calc, item_key, ospec, &inv_local, None);
            let immediate_amount = opc.iter_amounts().next().and_then(|v| match v.time == OF(0.0) {
                true => Some(v.amount),
                false => None,
            });
            events.push(CapSimEvent::InjectorReady(CapSimEventInjector {
                time: OF(0.0),
                cycle_iter: cseq.convert().iter_cycles(),
                opc,
                immediate_amount,
            }));
        }
    }
}
