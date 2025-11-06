use std::collections::BinaryHeap;

use super::{
    event_iter::{CapSimEventCycle, CapSimIterEvent},
    event_shared::CapSimEventCapGain,
    event_sim::CapSimEvent,
};
use crate::{
    def::{AttrVal, OF},
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
};

pub(super) struct CapSimIter {
    events: BinaryHeap<CapSimIterEvent>,
    injectors: Vec<(Cycle, AttrVal)>,
}
impl CapSimIter {
    pub(super) fn new(
        ctx: SvcCtx,
        calc: &mut Calc,
        vast: &Vast,
        fit_data: &VastFitData,
        cap_item_key: UItemKey,
    ) -> Self {
        let mut injectors = Vec::new();
        let mut events = BinaryHeap::new();
        // Consumers
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
                events.push(CapSimIterEvent::Cycle(CapSimEventCycle {
                    time: OF(0.0),
                    cycle_iter: effect_cycles.iter_cycles(),
                    output: output_per_cycle,
                }));
            }
        }
        // Neuts
        if let Some(neut_data) = vast.in_neuts.get_l1(&cap_item_key) {
            for (&neut_item_key, item_data) in neut_data.iter() {
                let mut cycle_map = match get_item_cycle_info(ctx, calc, neut_item_key, CYCLE_OPTIONS_BURST, false) {
                    Some(cycle_map) => cycle_map,
                    None => continue,
                };
                for (&effect_key, cap_getter) in item_data.iter() {
                    let effect = ctx.u_data.src.get_effect(effect_key);
                    let output_per_cycle = match cap_getter(ctx, calc, neut_item_key, effect, Some(cap_item_key)) {
                        Some(output_per_cycle) if output_per_cycle.has_impact() => output_per_cycle,
                        _ => continue,
                    };
                    let effect_cycles = match cycle_map.remove(&effect_key) {
                        Some(effect_cycles) => effect_cycles,
                        None => continue,
                    };
                    events.push(CapSimIterEvent::Cycle(CapSimEventCycle {
                        time: OF(0.0),
                        cycle_iter: effect_cycles.iter_cycles(),
                        output: -output_per_cycle,
                    }));
                }
            }
        };
        // Cap transfers
        if let Some(transfer_data) = vast.in_cap.get_l1(&cap_item_key) {
            for (&transfer_item_key, item_data) in transfer_data.iter() {
                let mut cycle_map = match get_item_cycle_info(ctx, calc, transfer_item_key, CYCLE_OPTIONS_BURST, false)
                {
                    Some(cycle_map) => cycle_map,
                    None => continue,
                };
                for (&effect_key, cap_getter) in item_data.iter() {
                    let effect = ctx.u_data.src.get_effect(effect_key);
                    let output_per_cycle =
                        match cap_getter(ctx, calc, transfer_item_key, effect, None, Some(cap_item_key)) {
                            Some(output_per_cycle) if output_per_cycle.has_impact() => output_per_cycle,
                            _ => continue,
                        };
                    let effect_cycles = match cycle_map.remove(&effect_key) {
                        Some(effect_cycles) => effect_cycles,
                        None => continue,
                    };
                    events.push(CapSimIterEvent::Cycle(CapSimEventCycle {
                        time: OF(0.0),
                        cycle_iter: effect_cycles.iter_cycles(),
                        output: output_per_cycle,
                    }));
                }
            }
        }
        // Cap injectors
        for (&item_key, item_data) in fit_data.cap_boosts.iter() {
            let mut cycle_map = match get_item_cycle_info(ctx, calc, item_key, CYCLE_OPTIONS_SIM, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            for (&effect_key, cap_getter) in item_data.iter() {
                let cap_injected = match cap_getter(ctx, calc, item_key) {
                    Some(cap_injected) if cap_injected != OF(0.0) => cap_injected,
                    _ => continue,
                };
                let effect_cycles = match cycle_map.remove(&effect_key) {
                    Some(effect_cycles) => effect_cycles,
                    None => continue,
                };
                injectors.push((effect_cycles, cap_injected));
            }
        }
        Self { events, injectors }
    }
}
impl Iterator for CapSimIter {
    type Item = CapSimEvent;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(event) = self.events.pop() {
            match event {
                CapSimIterEvent::Cycle(mut event) => {
                    // Add outputs for this cycle
                    let mut output_delay = OF(0.0);
                    for (output_interval, output_value) in event.output.iter_output() {
                        output_delay += output_interval;
                        let new_event = CapSimIterEvent::CapGain(CapSimEventCapGain {
                            time: event.time + output_delay,
                            amount: output_value,
                        });
                        self.events.push(new_event);
                    }
                    // Schedule next cycle, if any
                    if let Some(next_cycle_delay) = event.cycle_iter.next() {
                        let next_event = CapSimIterEvent::Cycle(CapSimEventCycle {
                            time: event.time + next_cycle_delay,
                            cycle_iter: event.cycle_iter,
                            output: event.output,
                        });
                        self.events.push(next_event);
                    }
                }
                CapSimIterEvent::InjectorAvailable(event) => return Some(CapSimEvent::InjectorAvailable(event)),
                CapSimIterEvent::CapGain(event) => {
                    return Some(CapSimEvent::CapGain(event));
                }
            }
        }
        None
    }
}
