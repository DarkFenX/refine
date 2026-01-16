use std::collections::BinaryHeap;

use super::event::{CapSimEvent, CapSimEventCapGain, CapSimEventCycleCheck, CapSimEventInjector};
use crate::{
    num::{PValue, UnitInterval, Value},
    svc::{output::OutputIterItem, vast::stats::shared::regenerate},
};

const TIME_LIMIT: PValue = PValue::from_f64_clamped(4.0 * 60.0 * 60.0);

pub enum StatCapSim {
    // Average stability value
    Stable(UnitInterval),
    // Time in seconds it takes to drain cap to 0
    Time(PValue),
}

pub(super) struct CapSim {
    max_cap: Value,
    max_pcap: PValue,
    tau: PValue,
    events: BinaryHeap<CapSimEvent>,
    // Injectors available for immediate use
    injectors: Vec<CapSimEventInjector>,
    // Current sim state
    time: PValue,
    cap: Value,
    only_gains: bool,
    wm_high_time: PValue,
    wm_high_cap: Value,
    wm_low_time: PValue,
    wm_low_cap: Value,
    wm_aux_high: Value,
    wm_aux_low: Value,
}
impl CapSim {
    pub(super) fn new(
        start_cap: PValue,
        max_cap: PValue,
        recharge_duration: PValue,
        events: BinaryHeap<CapSimEvent>,
    ) -> Self {
        Self {
            max_cap: max_cap.into_value(),
            max_pcap: max_cap,
            tau: recharge_duration / PValue::from_f64_unchecked(5.0),
            events,
            injectors: Vec::new(),
            time: PValue::ZERO,
            cap: start_cap.into_value(),
            only_gains: true,
            // Watermark data
            wm_high_time: PValue::ZERO,
            wm_high_cap: start_cap.into_value(),
            wm_low_time: PValue::ZERO,
            wm_low_cap: start_cap.into_value(),
            wm_aux_high: start_cap.into_value(),
            wm_aux_low: start_cap.into_value(),
        }
    }
    pub(super) fn run(&mut self) -> StatCapSim {
        while let Some(event) = self.events.pop() {
            match event {
                CapSimEvent::CycleCheck(mut event) => {
                    // Check if it can cycle altogether
                    if let Some(cycle_iter_info) = event.cycle_iter.next() {
                        // Add outputs for this cycle
                        self.schedule_cycle_output(event.time, event.opc.iter_amounts());
                        // Schedule next cycle check
                        let next_event = CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                            time: event.time + cycle_iter_info.duration,
                            cycle_iter: event.cycle_iter,
                            opc: event.opc,
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
                    match self.cap + event.immediate_amount.unwrap_or(Value::ZERO) > self.max_cap {
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
                    match event.amount >= Value::ZERO {
                        // Cap amount is increased
                        true => self.increase_cap(event.amount),
                        // Cap amount is decreased
                        false => {
                            if -event.amount > self.cap {
                                self.inject_emergency(-event.amount);
                            }
                            self.decrease_cap(event.amount);
                            if self.cap < Value::ZERO {
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
            return StatCapSim::Stable(UnitInterval::ONE);
        }
        // Instead of trying to detect event loops and averaging over looped period (which is
        // expensive), cap sim tracks global and auxiliary high and low watermarks. After new value
        // of high/low global watermark is reached, sim resets opposite auxiliary watermark. Final
        // stability value is average between last global watermark, and its opposite auxiliary
        // watermark
        let stability = match self.wm_high_time > self.wm_low_time {
            true => (self.wm_high_cap + self.wm_aux_low) / (Value::TWO * self.max_cap),
            false => (self.wm_low_cap + self.wm_aux_high) / (Value::TWO * self.max_cap),
        };
        // Extra checks for case when max cap is 0
        StatCapSim::Stable(UnitInterval::from_value_clamped(match stability.is_finite() {
            true => stability,
            false => Value::ONE,
        }))
    }
    fn advance_time(&mut self, new_time: PValue) {
        if new_time > self.time {
            self.cap = regenerate(
                PValue::from_value_unchecked(self.cap),
                self.max_pcap,
                self.tau,
                self.time,
                new_time,
            )
            .into_value();
            self.time = new_time;
            self.process_high_watermark();
        }
    }
    fn increase_cap(&mut self, amount: Value) {
        self.cap += amount;
        self.cap = Value::min(self.cap, self.max_cap);
        self.process_high_watermark();
    }
    fn decrease_cap(&mut self, amount: Value) {
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
    fn schedule_cycle_output(&mut self, base_time: PValue, output_iter: impl Iterator<Item = OutputIterItem<Value>>) {
        let mut extra_delay = PValue::ZERO;
        for output_event in output_iter {
            extra_delay += output_event.time_passed;
            let new_event = CapSimEvent::CapGain(CapSimEventCapGain {
                time: base_time + extra_delay,
                amount: output_event.amount,
            });
            self.events.push(new_event);
        }
    }
    fn use_injector(&mut self, mut injector_event: CapSimEventInjector) {
        // Check if injector can cycle
        if let Some(cycle_iter_info) = injector_event.cycle_iter.next() {
            // If injector has immediate effect, update cap and advance output amount iterator
            let mut output_iter = injector_event.opc.iter_amounts();
            if let Some(immediate_amount) = injector_event.immediate_amount {
                self.increase_cap(immediate_amount);
                output_iter.next();
            }
            // Schedule non-immediate cap change events (EVE injectors don't have that, but data
            // format used in the lib makes it possible)
            self.schedule_cycle_output(self.time, output_iter);
            // Schedule next cycle
            injector_event.time = self.time + cycle_iter_info.duration;
            self.events.push(CapSimEvent::InjectorReady(injector_event));
        }
    }
    fn inject_emergency(&mut self, needed_cap_total: Value) {
        while !self.injectors.is_empty() && needed_cap_total > self.cap && self.max_cap > self.cap {
            let needed_cap_extra = Value::min(needed_cap_total - self.cap, self.max_cap - self.cap);
            // Take injector which either provides just enough or more cap than needed
            let idx = match self
                .injectors
                .iter()
                .enumerate()
                .filter(|(_, v)| v.immediate_amount.unwrap_or(Value::ZERO) >= needed_cap_extra)
                .min_by_key(|(_, v)| v.immediate_amount.unwrap_or(Value::ZERO))
                .map(|(i, _)| i)
            {
                Some(idx) => idx,
                // If there are no such injectors, just take injector which provides the most cap
                None => self
                    .injectors
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, v)| v.immediate_amount.unwrap_or(Value::ZERO))
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
                .filter(|(_, v)| v.immediate_amount.unwrap_or(Value::ZERO) <= max_injection)
                .max_by_key(|(_, v)| v.immediate_amount.unwrap_or(Value::ZERO))
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
