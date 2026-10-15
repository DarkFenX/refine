use std::collections::{BinaryHeap, binary_heap::PeekMut};

use super::event::{CapSimEvent, CapSimEventCapChange, CapSimEventData, CapSimEventInjector};
use crate::{
    PValue, UnitInterval, Value,
    svc::{
        output::{Output, OutputInstanceIter},
        vast::stats::{cap::sim::shared::Direction, shared::regenerate},
    },
};

const TIME_LIMIT: PValue = PValue::from_f64_clamped(4.0 * 60.0 * 60.0);

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum StatCapSim {
    /// Average stability value
    Stable(UnitInterval),
    /// Time in seconds it takes to drain cap to 0
    Time(PValue),
}

pub(super) struct CapSim {
    max_cap: Value,
    max_pcap: PValue,
    tau: Option<PValue>,
    exp_cache: ExpCache,
    events: BinaryHeap<CapSimEvent>,
    /// Cap change events which happen at the time of event being processed. They are kept out of
    /// the heap, since they would be pushed to it and popped from it right away.
    current_events: Vec<CapSimEvent>,
    /// Delayed cap change events of cycle being processed, which wait until cycle check event is
    /// updated in the heap.
    delayed_events: Vec<CapSimEvent>,
    /// Injectors available for immediate use
    #[expect(clippy::vec_box)]
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
            exp_cache: ExpCache::new(),
            events,
            current_events: Vec::new(),
            delayed_events: Vec::new(),
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
        loop {
            let event = match self.get_next_event_source() {
                Some(EventSource::Heap) => {
                    let mut heap_event = self.events.peek_mut().unwrap();
                    let event_time = heap_event.get_time();
                    match &mut heap_event.data {
                        // Cycle check events are updated in place, to avoid removing them from the
                        // heap and adding them back
                        CapSimEventData::CycleCheck(event_data) => {
                            // Cap change which happens at the end of previous cycle
                            if let Some(cycle_end_event) = event_data.cycle_end_event.take() {
                                self.current_events.push(cycle_end_event);
                            }
                            // Check if it can cycle altogether
                            match event_data.cycle_iter.next() {
                                Some(cycle_iter_item) => {
                                    let next_time = event_time + cycle_iter_item.cycle_duration;
                                    let direction = event_data.direction;
                                    let cycle_end_event = &mut event_data.cycle_end_event;
                                    // Add outputs for this cycle
                                    Self::schedule_cycle_output(
                                        &mut self.current_events,
                                        |event| match cycle_end_event.is_none() && event.get_time() == next_time {
                                            true => *cycle_end_event = Some(event),
                                            false => self.delayed_events.push(event),
                                        },
                                        event_time,
                                        cycle_iter_item.output,
                                        cycle_iter_item.output_duration_limit,
                                        direction,
                                    );
                                    // Schedule next cycle check
                                    heap_event.set_time(next_time);
                                    drop(heap_event);
                                    for event in self.delayed_events.drain(..) {
                                        self.events.push(event);
                                    }
                                }
                                // When some module is done with cycling (non-repeating modules
                                // like CEHE), check if there are any events left, and if there are
                                // some, reset extra data used for stability value calculation.
                                None => {
                                    PeekMut::pop(heap_event);
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
                            continue;
                        }
                        _ => PeekMut::pop(heap_event),
                    }
                }
                Some(EventSource::Current(idx)) => self.current_events.swap_remove(idx),
                None => break,
            };
            let event_time = event.get_time();
            match event.data {
                // Cycle checks are processed in heap
                CapSimEventData::CycleCheck(..) => unreachable!(),
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
                let decay = self.exp_cache.exp((self.time - new_time) / tau);
                self.cap = regenerate(PValue::from_value_unchecked(self.cap), self.max_pcap, decay).into_value()
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
        current_events: &mut Vec<CapSimEvent>,
        mut schedule_delayed: impl FnMut(CapSimEvent),
        base_time: PValue,
        output: Output<PValue>,
        output_duration_limit: Option<PValue>,
        direction: Direction,
    ) {
        match output {
            // Simple output has just one instance, so shortcut is used - those instances are
            // scheduled without instance iterator
            Output::Simple(simple_output) => {
                Self::schedule_instance(
                    current_events,
                    &mut schedule_delayed,
                    base_time,
                    simple_output.delay,
                    simple_output.instance,
                    output_duration_limit,
                    direction,
                );
            }
            // Complex ones are using full-blown iterator with instance processing
            Output::Complex(..) => Self::schedule_output_instances(
                current_events,
                schedule_delayed,
                base_time,
                output.into_instance_iter(),
                output_duration_limit,
                direction,
            ),
        }
    }
    fn schedule_output_instances(
        current_events: &mut Vec<CapSimEvent>,
        mut schedule_delayed: impl FnMut(CapSimEvent),
        base_time: PValue,
        output_iter: OutputInstanceIter<PValue>,
        output_duration_limit: Option<PValue>,
        direction: Direction,
    ) {
        let mut delay = PValue::ZERO;
        for output_event in output_iter {
            delay += output_event.time_passed;
            if !Self::schedule_instance(
                current_events,
                &mut schedule_delayed,
                base_time,
                delay,
                output_event.instance,
                output_duration_limit,
                direction,
            ) {
                return;
            }
        }
    }
    // Returns false if instance happens after output duration limit, and was not scheduled because
    // of that
    #[inline(always)]
    fn schedule_instance(
        current_events: &mut Vec<CapSimEvent>,
        schedule_delayed: &mut impl FnMut(CapSimEvent),
        base_time: PValue,
        delay: PValue,
        instance: PValue,
        output_duration_limit: Option<PValue>,
        direction: Direction,
    ) -> bool {
        if let Some(output_duration_limit) = output_duration_limit
            && delay > output_duration_limit
        {
            return false;
        }
        let event = CapSimEvent::new(
            base_time + delay,
            CapSimEventData::CapChange(CapSimEventCapChange {
                amount: instance,
                direction,
            }),
        );
        // Base time is time of event being processed
        match delay == PValue::ZERO {
            true => current_events.push(event),
            false => schedule_delayed(event),
        }
        true
    }
    fn get_next_event_source(&self) -> Option<EventSource> {
        // Give priority for "current" time events
        let Some((current_idx, current_event)) =
            self.current_events.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b))
        else {
            return match self.events.is_empty() {
                true => None,
                false => Some(EventSource::Heap),
            };
        };
        // Not all the "current" time events are stored in special container, so double-check in the
        // heap
        match self.events.peek() {
            Some(heap_event) if heap_event > current_event => Some(EventSource::Heap),
            _ => Some(EventSource::Current(current_idx)),
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
            Self::schedule_output_instances(
                &mut self.current_events,
                |event| self.events.push(event),
                self.time,
                instance_iter,
                cycle_iter_item.output_duration_limit,
                Direction::Gain,
            );
            // Schedule next cycle
            self.events.push(CapSimEvent::new(
                self.time + cycle_iter_item.cycle_duration,
                CapSimEventData::InjectorReady(injector_data),
            ));
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
        match self.events.is_empty() && self.current_events.is_empty() {
            // When there are no events, return 100% if there is some cap regen, or initial cap
            // value if cap is not regenerating
            true => Some(match self.tau {
                Some(..) => StatCapSim::Stable(UnitInterval::ONE),
                None => StatCapSim::Stable(UnitInterval::from_value_clamped(self.cap / self.max_cap)),
            }),
            false => None,
        }
    }
}

enum EventSource {
    Heap,
    Current(usize),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regen cache
////////////////////////////////////////////////////////////////////////////////////////////////////
const CACHE_SIZE: usize = 8;

/// Cache of exp() results for recently used arguments, which repeat a lot in sim, since time deltas
/// between events repeat.
struct ExpCache {
    // Bits of arguments, initialized with +0.0, which is never passed, since arguments are always
    // negative
    args: [u64; CACHE_SIZE],
    results: [PValue; CACHE_SIZE],
    next: usize,
}
impl ExpCache {
    fn new() -> Self {
        Self {
            args: [0; CACHE_SIZE],
            results: [PValue::ZERO; CACHE_SIZE],
            next: 0,
        }
    }
    fn exp(&mut self, arg: Value) -> PValue {
        let arg_bits = arg.into_f64().to_bits();
        if let Some(idx) = self.args.iter().position(|&v| v == arg_bits) {
            return self.results[idx];
        }
        let result = arg.exp();
        self.args[self.next] = arg_bits;
        self.results[self.next] = result;
        self.next = (self.next + 1) % CACHE_SIZE;
        result
    }
}
