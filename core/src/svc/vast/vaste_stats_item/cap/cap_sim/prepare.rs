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
        calc::Calc,
        cycle::get_item_cycle_info,
        output::{Output, OutputSimple},
        vast::{
            Vast, VastFitData,
            vaste_stats_item::cap::shared::{CYCLE_OPTIONS_BURST, CYCLE_OPTIONS_SIM},
        },
    },
    ud::UItemKey,
    util::{FLOAT_TOLERANCE, RMapVec},
};

pub(super) fn prepare_events(
    ctx: SvcCtx,
    calc: &mut Calc,
    vast: &Vast,
    stagger: StatCapSimStaggerInt,
    fit_data: &VastFitData,
    cap_item_key: UItemKey,
) -> BinaryHeap<CapSimEvent> {
    let mut aggregator = Aggregator::new();
    fill_consumers(ctx, calc, &mut aggregator, &stagger, fit_data);
    fill_neuts(ctx, calc, &mut aggregator, &stagger, vast, cap_item_key);
    fill_transfers(ctx, calc, &mut aggregator, &stagger, vast, cap_item_key);
    let mut events = BinaryHeap::new();
    aggregator.into_sim_events(&mut events);
    fill_injectors(ctx, calc, &mut events, fit_data);
    events
}

fn fill_consumers(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    stagger: &StatCapSimStaggerInt,
    fit_data: &VastFitData,
) {
    let mut stagger_map = RMapVec::new();
    for (&item_key, item_data) in fit_data.cap_consumers_active.iter() {
        let mut cycle_map = match get_item_cycle_info(ctx, calc, item_key, CYCLE_OPTIONS_SIM, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, &attr_key) in item_data.iter() {
            let cap_consumed = match calc.get_item_attr_oextra(ctx, item_key, attr_key) {
                // Cap consumed can be negative value, e.g. for nosfs
                Some(cap_consumed) if cap_consumed.abs() > FLOAT_TOLERANCE => cap_consumed,
                _ => continue,
            };
            let effect_cycles = match cycle_map.remove(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let output_per_cycle = Output::Simple(OutputSimple {
                amount: -cap_consumed,
                delay: OF(0.0),
            });
            match stagger.is_staggered(item_key) {
                true => stagger_map.add_entry(
                    StaggerKey::new(&effect_cycles, &output_per_cycle),
                    (effect_cycles, output_per_cycle),
                ),
                false => aggregator.add_entry(OF(0.0), effect_cycles, output_per_cycle),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_neuts(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    stagger: &StatCapSimStaggerInt,
    vast: &Vast,
    cap_item_key: UItemKey,
) {
    let neut_data = match vast.in_neuts.get_l1(&cap_item_key) {
        Some(neut_data) => neut_data,
        None => return,
    };
    let mut stagger_map = RMapVec::new();
    for (&neut_item_key, item_data) in neut_data.iter() {
        let mut cycle_map = match get_item_cycle_info(ctx, calc, neut_item_key, CYCLE_OPTIONS_BURST, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, cap_getter) in item_data.iter() {
            let effect = ctx.u_data.src.get_effect(effect_key);
            let output_per_cycle = match cap_getter(ctx, calc, neut_item_key, effect, Some(cap_item_key)) {
                // Negate output, since neuts negatively impact cap, but output of neut getter
                // function is positive
                Some(output_per_cycle) if output_per_cycle.has_impact() => -output_per_cycle,
                _ => continue,
            };
            let effect_cycles = match cycle_map.remove(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            match stagger.is_staggered(neut_item_key) {
                true => stagger_map.add_entry(
                    StaggerKey::new(&effect_cycles, &output_per_cycle),
                    (effect_cycles, output_per_cycle),
                ),
                false => aggregator.add_entry(OF(0.0), effect_cycles, output_per_cycle),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_transfers(
    ctx: SvcCtx,
    calc: &mut Calc,
    aggregator: &mut Aggregator,
    stagger: &StatCapSimStaggerInt,
    vast: &Vast,
    cap_item_key: UItemKey,
) {
    let transfer_data = match vast.in_cap.get_l1(&cap_item_key) {
        Some(transfer_data) => transfer_data,
        None => return,
    };
    let mut stagger_map = RMapVec::new();
    for (&transfer_item_key, item_data) in transfer_data.iter() {
        let mut cycle_map = match get_item_cycle_info(ctx, calc, transfer_item_key, CYCLE_OPTIONS_BURST, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, cap_getter) in item_data.iter() {
            let effect = ctx.u_data.src.get_effect(effect_key);
            let output_per_cycle = match cap_getter(ctx, calc, transfer_item_key, effect, None, Some(cap_item_key)) {
                Some(output_per_cycle) if output_per_cycle.has_impact() => output_per_cycle,
                _ => continue,
            };
            let effect_cycles = match cycle_map.remove(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            match stagger.is_staggered(transfer_item_key) {
                true => stagger_map.add_entry(
                    StaggerKey::new(&effect_cycles, &output_per_cycle),
                    (effect_cycles, output_per_cycle),
                ),
                false => aggregator.add_entry(OF(0.0), effect_cycles, output_per_cycle),
            }
        }
    }
    process_staggers(stagger_map, aggregator);
}

fn fill_injectors(ctx: SvcCtx, calc: &mut Calc, events: &mut BinaryHeap<CapSimEvent>, fit_data: &VastFitData) {
    for (&item_key, item_data) in fit_data.cap_injects.iter() {
        let mut cycle_map = match get_item_cycle_info(ctx, calc, item_key, CYCLE_OPTIONS_SIM, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, cap_getter) in item_data.iter() {
            let cap_injected = match cap_getter(ctx, calc, item_key) {
                // Even if some injector has negative value, player doesn't have to use it, so it is
                // just ignored
                Some(cap_injected) if cap_injected > FLOAT_TOLERANCE => cap_injected,
                _ => continue,
            };
            let effect_cycles = match cycle_map.remove(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            events.push(CapSimEvent::InjectorReady(CapSimEventInjector {
                time: OF(0.0),
                cycle_iter: effect_cycles.iter_cycles(),
                output: cap_injected,
            }));
        }
    }
}
