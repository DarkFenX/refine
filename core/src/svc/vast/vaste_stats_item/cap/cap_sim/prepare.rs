use std::collections::BinaryHeap;

use itertools::Itertools;

use super::{
    event::{CapSimEvent, CapSimEventCycleCheck, CapSimEventInjector},
    stagger::{StaggerKey, StatCapSimStaggerInt},
};
use crate::{
    AttrVal,
    def::OF,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, get_item_cycle_info},
        output::{Output, OutputSimple},
        vast::{
            Vast, VastFitData,
            vaste_stats_item::cap::shared::{CYCLE_OPTIONS_BURST, CYCLE_OPTIONS_SIM},
        },
    },
    ud::UItemKey,
    util::RMapVec,
};

pub(super) fn prepare_events(
    ctx: SvcCtx,
    calc: &mut Calc,
    vast: &Vast,
    stagger: StatCapSimStaggerInt,
    fit_data: &VastFitData,
    cap_item_key: UItemKey,
) -> BinaryHeap<CapSimEvent> {
    let mut events = BinaryHeap::new();
    fill_consumers(ctx, calc, &mut events, &stagger, fit_data);
    fill_neuts(ctx, calc, &mut events, &stagger, vast, cap_item_key);
    fill_transfers(ctx, calc, &mut events, &stagger, vast, cap_item_key);
    fill_injectors(ctx, calc, &mut events, fit_data);
    events
}

fn fill_consumers(
    ctx: SvcCtx,
    calc: &mut Calc,
    events: &mut BinaryHeap<CapSimEvent>,
    stagger: &StatCapSimStaggerInt,
    fit_data: &VastFitData,
) {
    let mut stagger_map = RMapVec::new();
    for (&item_key, item_data) in fit_data.cap_consumers.iter() {
        let mut cycle_map = match get_item_cycle_info(ctx, calc, item_key, CYCLE_OPTIONS_SIM, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, attr_id) in item_data.iter() {
            let cap_used = match calc.get_item_attr_val_extra(ctx, item_key, attr_id) {
                Ok(cap_used) if cap_used != OF(0.0) => cap_used,
                _ => continue,
            };
            let effect_cycles = match cycle_map.remove(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let output_per_cycle = Output::Simple(OutputSimple {
                amount: -cap_used,
                delay: OF(0.0),
            });
            match stagger.is_staggered(item_key) {
                true => stagger_map.add_entry(
                    StaggerKey::new(&effect_cycles, &output_per_cycle),
                    (effect_cycles, output_per_cycle),
                ),
                false => events.push(CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                    time: OF(0.0),
                    cycle_iter: effect_cycles.iter_cycles(),
                    output: output_per_cycle,
                })),
            }
        }
    }
    process_staggers(stagger_map, events);
}

fn fill_neuts(
    ctx: SvcCtx,
    calc: &mut Calc,
    events: &mut BinaryHeap<CapSimEvent>,
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
                false => events.push(CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                    time: OF(0.0),
                    cycle_iter: effect_cycles.iter_cycles(),
                    output: output_per_cycle,
                })),
            }
        }
    }
    process_staggers(stagger_map, events);
}

fn fill_transfers(
    ctx: SvcCtx,
    calc: &mut Calc,
    events: &mut BinaryHeap<CapSimEvent>,
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
                false => events.push(CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                    time: OF(0.0),
                    cycle_iter: effect_cycles.iter_cycles(),
                    output: output_per_cycle,
                })),
            }
        }
    }
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
                Some(cap_injected) if cap_injected > OF(0.0) => cap_injected,
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

fn process_staggers(stagger_map: RMapVec<StaggerKey, (Cycle, Output<AttrVal>)>, events: &mut BinaryHeap<CapSimEvent>) {
    for (stagger_key, stagger_group) in stagger_map.into_iter() {
        if stagger_group.len() < 2 {
            for (cycles, output) in stagger_group.into_iter() {
                events.push(CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                    time: OF(0.0),
                    cycle_iter: cycles.iter_cycles(),
                    output,
                }));
            }
            continue;
        }
        // Sort by output value, from highest to lowest
        let stagger_period = stagger_key.cycle.get_cycle_time_for_stagger() / stagger_group.len() as f64;
        for (i, (cycles, output)) in stagger_group
            .into_iter()
            .sorted_by_key(|(_, o)| -o.absolute_impact())
            .enumerate()
        {
            events.push(CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                time: stagger_period * i as f64,
                cycle_iter: cycles.iter_cycles(),
                output,
            }))
        }
    }
}
