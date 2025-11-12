use std::collections::BinaryHeap;

use ordered_float::Float;

use super::event::{CapSimEvent, CapSimEventCapGain, CapSimEventCycleCheck, CapSimEventInjector};
use crate::{
    def::{AttrVal, OF},
    util::UnitInterval,
};

const TIME_LIMIT: AttrVal = OF(6.0 * 60.0 * 60.0);

pub enum StatCapSim {
    // Average stability value
    Stable(UnitInterval),
    // Time in seconds it takes to drain cap to 0
    Time(AttrVal),
}

pub(super) struct CapSim {
    max_cap: AttrVal,
    tau: AttrVal,
    events: BinaryHeap<CapSimEvent>,
    // Injectors available for immediate use
    injectors: Vec<CapSimEventInjector>,
    // Current sim state
    time: AttrVal,
    cap: AttrVal,
    only_gains: bool,
    wm_high_time: AttrVal,
    wm_high_cap: AttrVal,
    wm_low_time: AttrVal,
    wm_low_cap: AttrVal,
    wm_aux_high: AttrVal,
    wm_aux_low: AttrVal,
}
impl CapSim {
    pub(super) fn new(
        start_cap: AttrVal,
        max_cap: AttrVal,
        recharge_time: AttrVal,
        events: BinaryHeap<CapSimEvent>,
    ) -> Self {
        Self {
            max_cap,
            tau: recharge_time / OF(5.0),
            events,
            injectors: Vec::new(),
            time: OF(0.0),
            cap: start_cap,
            only_gains: true,
            // Watermark data
            wm_high_time: OF(0.0),
            wm_high_cap: start_cap,
            wm_low_time: OF(0.0),
            wm_low_cap: start_cap,
            wm_aux_high: start_cap,
            wm_aux_low: start_cap,
        }
    }
    pub(super) fn run(&mut self) -> StatCapSim {
        while let Some(event) = self.events.pop() {
            match event {
                CapSimEvent::CycleCheck(mut event) => {
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
                            self.events.push(new_event);
                        }
                        // Schedule next cycle check
                        let next_event = CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                            time: event.time + next_cycle_delay,
                            cycle_iter: event.cycle_iter,
                            output: event.output,
                        });
                        self.events.push(next_event);
                    }
                }
                CapSimEvent::InjectorReady(event) => {
                    // Update basic sim state according to time progression
                    if event.time > TIME_LIMIT {
                        self.advance_time(TIME_LIMIT);
                        break;
                    }
                    self.advance_time(event.time);
                    // Use injector right away if it does not overshoot cap, or postpone if it does
                    match self.cap + event.output > self.max_cap {
                        true => self.injectors.push(event),
                        false => self.use_injector(event),
                    }
                }
                CapSimEvent::CapGain(event) => {
                    // Update basic sim state according to time progression
                    if event.time > TIME_LIMIT {
                        self.advance_time(TIME_LIMIT);
                        break;
                    }
                    self.advance_time(event.time);
                    // Process cap change from event
                    match event.amount >= OF(0.0) {
                        // Cap amount is increased
                        true => self.increase_cap(event.amount),
                        // Cap amount is decreased
                        false => {
                            if -event.amount > self.cap {
                                self.inject_emergency(-event.amount);
                            }
                            self.decrease_cap(event.amount);
                            if self.cap < OF(0.0) {
                                return StatCapSim::Time(self.time);
                            }
                            // After some cap was removed, check if we can top up using injector
                            self.inject_topup();
                        }
                    }
                }
            }
        }
        // No drains - cap regens up to 100% even if no other gains are registered
        if self.only_gains {
            return StatCapSim::Stable(UnitInterval::new_clamped_of64(OF(1.0)));
        }
        // Instead of trying to detect event loops and averaging over looped period (which is
        // expensive), cap sim tracks global and auxiliary high and low watermarks. After new value
        // of high/low global watermark is reached, sim resets opposite auxiliary watermark. Final
        // stability value is average between last global watermark, and its opposite auxiliary
        // watermark
        let stability = match self.wm_high_time > self.wm_low_time {
            true => (self.wm_high_cap + self.wm_aux_low) / (OF(2.0) * self.max_cap),
            false => (self.wm_low_cap + self.wm_aux_high) / (OF(2.0) * self.max_cap),
        };
        // Extra checks for case when max cap is 0
        StatCapSim::Stable(UnitInterval::new_clamped_of64(match stability.is_finite() {
            true => stability,
            false => OF(1.0),
        }))
    }
    fn advance_time(&mut self, new_time: AttrVal) {
        if new_time > self.time {
            self.cap = calc_regen(self.cap, self.max_cap, self.tau, self.time, new_time);
            self.time = new_time;
            self.process_high_watermark();
        }
    }
    fn increase_cap(&mut self, amount: AttrVal) {
        self.cap += amount;
        self.cap = Float::min(self.cap, self.max_cap);
        self.process_high_watermark();
    }
    fn decrease_cap(&mut self, amount: AttrVal) {
        self.cap += amount;
        self.only_gains = false;
        self.process_low_watermark();
    }
    fn process_high_watermark(&mut self) {
        if self.cap > self.wm_high_cap {
            self.wm_high_time = self.time;
            self.wm_high_cap = self.cap;
            // Each time new high watermark is recorded, reset auxiliary low watermark
            self.wm_aux_low = self.cap;
        }
        if self.cap > self.wm_aux_high {
            self.wm_aux_high = self.cap;
        }
    }
    fn process_low_watermark(&mut self) {
        if self.cap < self.wm_low_cap {
            self.wm_low_time = self.time;
            self.wm_low_cap = self.cap;
            // Each time new low watermark is recorded, reset auxiliary high watermark
            self.wm_aux_high = self.cap;
        }
        if self.cap < self.wm_aux_low {
            self.wm_aux_low = self.cap;
        }
    }
    fn use_injector(&mut self, mut injector_event: CapSimEventInjector) {
        // Check if injector can cycle
        if let Some(next_cycle_delay) = injector_event.cycle_iter.next() {
            // If it can, update cap value
            self.increase_cap(injector_event.output);
            // Schedule next cycle
            injector_event.time = self.time + next_cycle_delay;
            self.events.push(CapSimEvent::InjectorReady(injector_event));
        }
    }
    fn inject_emergency(&mut self, needed_cap_total: AttrVal) {
        while !self.injectors.is_empty() && needed_cap_total > self.cap && self.max_cap > self.cap {
            let needed_cap_extra = Float::min(needed_cap_total - self.cap, self.max_cap - self.cap);
            // Take injector which either provides just enough or more cap than needed
            let idx = match self
                .injectors
                .iter()
                .enumerate()
                .filter(|(_, v)| v.output >= needed_cap_extra)
                .min_by_key(|(_, v)| v.output)
                .map(|(i, _)| i)
            {
                Some(idx) => idx,
                // If there are no such injectors, just take injector which provides the most cap
                None => self
                    .injectors
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, v)| v.output)
                    .map(|(i, _)| i)
                    .unwrap(),
            };
            let injector = self.injectors.remove(idx);
            self.use_injector(injector);
        }
    }
    fn inject_topup(&mut self) {
        while !self.injectors.is_empty() && self.cap < self.max_cap {
            let max_injection = self.max_cap - self.cap;
            // Find an injector which does not overshoot and has the highest injection value
            let idx = match self
                .injectors
                .iter()
                .enumerate()
                .filter(|(_, v)| v.output <= max_injection)
                .max_by_key(|(_, v)| v.output)
                .map(|(i, _)| i)
            {
                Some(idx) => idx,
                None => return,
            };
            let injector = self.injectors.remove(idx);
            self.use_injector(injector);
        }
    }
}

fn calc_regen(c0: AttrVal, c_max: AttrVal, tau: AttrVal, t0: AttrVal, t1: AttrVal) -> AttrVal {
    (OF(1.0) + ((c0 / c_max).sqrt() - OF(1.0)) * ((t0 - t1) / tau).exp()).powi(2) * c_max
}
