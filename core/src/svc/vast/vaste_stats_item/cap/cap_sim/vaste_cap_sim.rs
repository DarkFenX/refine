use std::collections::BinaryHeap;

use ordered_float::Float;

use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{
            Vast,
            vaste_stats_item::{
                cap::cap_sim::{
                    event::{CapSimEvent, CapSimEventCapGain, CapSimEventCycle, CapSimEventInjector},
                    prepare::prepare_events,
                },
                checks::check_item_ship,
            },
        },
    },
    ud::UItemKey,
};

const TIME_LIMIT: AttrVal = OF(4.0 * 60.0 * 60.0);

pub enum StatCapSim {
    // Low watermark of stability value
    Stable(AttrVal),
    // Time in seconds it takes to drain cap to 0
    Time(AttrVal),
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_sim(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        cap_perc: Option<AttrVal>,
    ) -> Result<StatCapSim, StatItemCheckError> {
        let item = ctx.u_data.items.get(item_key);
        check_item_ship(item_key, item)?;
        let max_cap = Vast::get_stat_item_cap_amount(ctx, calc, item_key).unwrap();
        let tau = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::RECHARGE_RATE)
            .unwrap()
            / OF(5000.0);
        let mut sim_time = OF(0.0);
        let mut sim_cap = match cap_perc {
            Some(perc) => max_cap * perc,
            None => max_cap,
        };
        // Injectors available for immediate use
        let fit_data = self.fit_datas.get(&item.get_ship().unwrap().get_fit_key()).unwrap();
        let mut events = prepare_events(ctx, calc, self, fit_data, item_key);
        let mut injectors = Vec::new();
        while let Some(event) = events.pop() {
            match event {
                CapSimEvent::Cycle(mut event) => {
                    // Check if it can cycle altogether
                    if let Some(next_cycle_delay) = event.cycle_iter.next() {
                        // Add outputs for this cycle
                        let mut output_delay = OF(0.0);
                        for (output_interval, output_value) in event.output.iter_output() {
                            output_delay += output_interval;
                            let new_event = CapSimEvent::CapGain(CapSimEventCapGain {
                                time: event.time + output_delay,
                                amount: output_value,
                            });
                            events.push(new_event);
                        }
                        // Schedule next cycle check
                        let next_event = CapSimEvent::Cycle(CapSimEventCycle {
                            time: event.time + next_cycle_delay,
                            cycle_iter: event.cycle_iter,
                            output: event.output,
                        });
                        events.push(next_event);
                    }
                }
                CapSimEvent::InjectorReady(event) => {
                    // Update basic sim state according to time progression
                    if event.time > TIME_LIMIT {
                        break;
                    }
                    if event.time > sim_time {
                        sim_cap = calc_regen(sim_cap, max_cap, tau, sim_time, event.time);
                        sim_time = event.time;
                    }
                    // Use injector right away if it does not overshoot cap, or postpone if it does
                    match sim_cap + event.output > max_cap {
                        true => injectors.push(event),
                        false => {
                            use_injector(sim_time, &mut sim_cap, max_cap, event, &mut events);
                        }
                    }
                }
                CapSimEvent::CapGain(event) => {
                    // Update basic sim state according to time progression
                    if event.time > TIME_LIMIT {
                        break;
                    }
                    if event.time > sim_time {
                        sim_cap = calc_regen(sim_cap, max_cap, tau, sim_time, event.time);
                        sim_time = event.time;
                    }
                    // Process cap change from event
                    match event.amount >= OF(0.0) {
                        true => {
                            sim_cap += event.amount;
                            sim_cap = Float::min(sim_cap, max_cap);
                        }
                        false => {
                            sim_cap += event.amount;
                            if sim_cap < OF(0.0) {
                                return Ok(StatCapSim::Time(sim_time));
                            }
                            // After some cap was removed, check if we can top up using injector
                            top_up_after(sim_time, &mut sim_cap, max_cap, &mut injectors, &mut events);
                        }
                    }
                }
            }
        }
        Ok(StatCapSim::Stable(OF(0.25)))
    }
}

fn calc_regen(c0: AttrVal, c_max: AttrVal, tau: AttrVal, t0: AttrVal, t1: AttrVal) -> AttrVal {
    (OF(1.0) + ((c0 / c_max).sqrt() - OF(1.0)) * ((t0 - t1) / tau).exp()).powi(2) * c_max
}

fn use_injector(
    sim_time: AttrVal,
    sim_cap: &mut AttrVal,
    max_cap: AttrVal,
    mut injector_event: CapSimEventInjector,
    events: &mut BinaryHeap<CapSimEvent>,
) {
    // Check if injector can cycle
    if let Some(next_cycle_delay) = injector_event.cycle_iter.next() {
        // If it can, update cap value
        *sim_cap += injector_event.output;
        *sim_cap = Float::min(*sim_cap, max_cap);
        // Schedule next cycle
        injector_event.time = sim_time + next_cycle_delay;
        events.push(CapSimEvent::InjectorReady(injector_event));
    }
}

fn top_up_after(
    sim_time: AttrVal,
    sim_cap: &mut AttrVal,
    max_cap: AttrVal,
    injectors: &mut Vec<CapSimEventInjector>,
    events: &mut BinaryHeap<CapSimEvent>,
) {
    while !injectors.is_empty() && *sim_cap < max_cap {
        let max_injection = max_cap - *sim_cap;
        // Find an injector which does not overshoot and has the highest injection value
        let idx = match injectors
            .iter()
            .enumerate()
            .filter(|(_, v)| v.output <= max_injection)
            .max_by_key(|(_, v)| v.output)
            .map(|(i, _)| i)
        {
            Some(idx) => idx,
            None => return,
        };
        let injector = injectors.remove(idx);
        use_injector(sim_time, sim_cap, max_cap, injector, events);
    }
}
