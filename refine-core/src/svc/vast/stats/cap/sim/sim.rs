use std::collections::BinaryHeap;

use super::event::{CapSimEvent, CapSimEventCapChange, CapSimEventData, CapSimEventInjector};
use crate::{
    PValue, UnitInterval, Value,
    svc::{
        output::OutputInstanceIter,
        vast::stats::{cap::sim::shared::Direction, shared::regenerate},
    },
};

const TIME_LIMIT: PValue = PValue::from_f64_clamped(4.0 * 60.0 * 60.0);

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum StatCapSim {
    // Average stability value
    Stable(UnitInterval),
    // Time in seconds it takes to drain cap to 0
    Time(PValue),
}

pub(super) struct CapSim {
    max_cap: Value,
    max_pcap: PValue,
    tau: Option<PValue>,
    events: BinaryHeap<CapSimEvent>,
    // Injectors available for immediate use
    #[allow(clippy::vec_box)]
    injectors: Vec<Box<CapSimEventInjector>>,
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
        recharge_duration: Option<PValue>,
        events: BinaryHeap<CapSimEvent>,
    ) -> Self {
        Self {
            max_cap: max_cap.into_value(),
            max_pcap: max_cap,
            tau: recharge_duration.map(|v| v / PValue::from_f64_unchecked(5.0)),
            events,
            injectors: Vec::new(),
            time: PValue::ZERO,
            cap: start_cap.into_value(),
            // Data needed to calculate cap stability - watermarks and extra flags for edge cases
            only_gains: true,
            wm_high_time: PValue::ZERO,
            wm_high_cap: start_cap.into_value(),
            wm_low_time: PValue::ZERO,
            wm_low_cap: start_cap.into_value(),
            wm_aux_high: start_cap.into_value(),
            wm_aux_low: start_cap.into_value(),
        }
    }
    pub(super) fn run(&mut self) -> StatCapSim {
        if let Some(result) = self.check_no_events() {
            return result;
        }
        while let Some(event) = self.events.pop() {
            let event_time = event.time;
            match event.data {
                CapSimEventData::CycleCheck(mut event_data) => {
                    // Check if it can cycle altogether
                    match event_data.cycle_iter.next() {
                        Some(cycle_iter_item) => {
                            // Add outputs for this cycle
                            self.schedule_cycle_output(
                                event_time,
                                cycle_iter_item.output.into_instance_iter(),
                                cycle_iter_item.output_duration_limit,
                                event_data.direction,
                            );
                            // Schedule next cycle check, reusing allocation of the current event.
                            // Cycle iter and direction stay the same, only time needs updating
                            self.events.push(CapSimEvent {
                                time: event_time + cycle_iter_item.cycle_duration,
                                data: CapSimEventData::CycleCheck(event_data),
                            });
                        }
                        // When some module is done with cycling (non-repeating modules like CEHE),
                        // check if there are any events left, and if there are some, reset extra
                        // data used for stability value calculation.
                        None => {
                            if let Some(result) = self.check_no_events() {
                                return result;
                            }
                            self.only_gains = true;
                            self.wm_high_time = event_time;
                            self.wm_high_cap = self.cap;
                            self.wm_low_time = event_time;
                            self.wm_low_cap = self.cap;
                            self.wm_aux_high = self.cap;
                            self.wm_aux_low = self.cap;
                        }
                    }
                }
                CapSimEventData::InjectorReady(event_data) => {
                    // Update basic sim state according to time progression
                    if event_time > TIME_LIMIT {
                        self.advance_time(TIME_LIMIT);
                        break;
                    }
                    self.advance_time(event_time);
                    // Use injector right away if it does not overshoot cap, or postpone if it does
                    match self.cap + event_data.get_immediate_instance().unwrap_or(PValue::ZERO).into_value()
                        > self.max_cap
                    {
                        true => self.injectors.push(event_data),
                        false => self.use_injector(event_data),
                    }
                }
                CapSimEventData::CapChange(event_data) => {
                    // Update basic sim state according to time progression
                    if event_time > TIME_LIMIT {
                        self.advance_time(TIME_LIMIT);
                        break;
                    }
                    self.advance_time(event_time);
                    // Process cap change from event
                    match event_data.direction {
                        Direction::Gain if event_data.amount > PValue::ZERO => self.increase_cap(event_data.amount),
                        Direction::Loss if event_data.amount > PValue::ZERO => {
                            if event_data.amount.into_value() > self.cap {
                                self.inject_emergency(event_data.amount);
                            }
                            self.decrease_cap(event_data.amount);
                            if self.cap < Value::ZERO {
                                return StatCapSim::Time(self.time);
                            }
                            // After some cap was removed, check if we can top up using injector
                            self.inject_topup();
                        }
                        _ => (),
                    }
                }
            }
        }
        // There were some events, but only gains - expose 100% stability without using any
        // watermark logic
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
            if let Some(tau) = self.tau {
                self.cap = regenerate(
                    PValue::from_value_unchecked(self.cap),
                    self.max_pcap,
                    tau,
                    self.time,
                    new_time,
                )
                .into_value()
            };
            self.time = new_time;
            self.process_high_watermark();
        }
    }
    fn increase_cap(&mut self, amount: PValue) {
        self.cap += amount;
        self.cap = Value::min(self.cap, self.max_cap);
        self.process_high_watermark();
    }
    fn decrease_cap(&mut self, amount: PValue) {
        self.cap -= amount;
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
    fn schedule_cycle_output(
        &mut self,
        base_time: PValue,
        output_iter: OutputInstanceIter<PValue>,
        output_duration_limit: Option<PValue>,
        direction: Direction,
    ) {
        let mut extra_delay = PValue::ZERO;
        for output_event in output_iter {
            extra_delay += output_event.time_passed;
            if let Some(output_duration_limit) = output_duration_limit
                && extra_delay > output_duration_limit
            {
                return;
            }
            self.events.push(CapSimEvent {
                time: base_time + extra_delay,
                data: CapSimEventData::CapChange(CapSimEventCapChange {
                    amount: output_event.instance,
                    direction,
                }),
            });
        }
    }
    fn use_injector(&mut self, mut injector_data: Box<CapSimEventInjector>) {
        // Check if injector can cycle
        if let Some(cycle_iter_item) = injector_data.cycle_iter.next() {
            // If injector has immediate effect, update cap and advance output instance iterator
            let immediate_amount = cycle_iter_item.output.get_immediate_instance();
            let mut instance_iter = cycle_iter_item.output.into_instance_iter();
            if let Some(immediate_amount) = immediate_amount {
                self.increase_cap(immediate_amount);
                instance_iter.next();
            }
            // Schedule non-immediate cap change events (EVE injectors don't have that, but data
            // format used in the lib makes it possible)
            self.schedule_cycle_output(
                self.time,
                instance_iter,
                cycle_iter_item.output_duration_limit,
                Direction::Gain,
            );
            // Schedule next cycle
            self.events.push(CapSimEvent {
                time: self.time + cycle_iter_item.cycle_duration,
                data: CapSimEventData::InjectorReady(injector_data),
            });
        }
    }
    fn inject_emergency(&mut self, needed_cap_total: PValue) {
        while !self.injectors.is_empty() && needed_cap_total.into_value() > self.cap && self.max_cap > self.cap {
            let needed_cap_extra = PValue::from_value_clamped(Value::min(
                needed_cap_total.into_value() - self.cap,
                self.max_cap - self.cap,
            ));
            // Take injector which either provides just enough or more cap than needed
            let idx = match self
                .injectors
                .iter()
                .enumerate()
                .filter(|(_, v)| v.get_immediate_instance().unwrap_or(PValue::ZERO) >= needed_cap_extra)
                .min_by_key(|(_, v)| v.get_immediate_instance().unwrap_or(PValue::ZERO))
                .map(|(i, _)| i)
            {
                Some(idx) => idx,
                // If there are no such injectors, just take injector which provides the most cap
                None => self
                    .injectors
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, v)| v.get_immediate_instance().unwrap_or(PValue::ZERO))
                    .map(|(i, _)| i)
                    .unwrap(),
            };
            let injector = self.injectors.swap_remove(idx);
            self.use_injector(injector);
        }
    }
    fn inject_topup(&mut self) {
        while !self.injectors.is_empty() && self.cap < self.max_cap {
            let max_injection = PValue::from_value_clamped(self.max_cap - self.cap);
            // Find an injector which does not overshoot and has the highest injection value
            let Some(idx) = self
                .injectors
                .iter()
                .enumerate()
                .filter(|(_, v)| v.get_immediate_instance().unwrap_or(PValue::ZERO) <= max_injection)
                .max_by_key(|(_, v)| v.get_immediate_instance().unwrap_or(PValue::ZERO))
                .map(|(i, _)| i)
            else {
                return;
            };
            let injector = self.injectors.swap_remove(idx);
            self.use_injector(injector);
        }
    }
    fn check_no_events(&self) -> Option<StatCapSim> {
        match self.events.is_empty() {
            // When there are no events, return 100% if there is some cap regen, or initial cap
            // value if cap is not regenerating
            true => Some(match self.tau {
                Some(_) => StatCapSim::Stable(UnitInterval::ONE),
                None => StatCapSim::Stable(UnitInterval::from_value_clamped(self.cap / self.max_cap)),
            }),
            false => None,
        }
    }
}
