use std::collections::BinaryHeap;

use super::event::{CapSimEvent, CapSimEventCycleCheck};
use crate::{
    def::{AttrVal, Count, OF},
    svc::{
        cycle::Cycle,
        output::{Output, OutputComplex, OutputSimple},
    },
    util::{RMapVec, sig_round},
};

pub(super) struct Aggregator {
    data: RMapVec<AggrKey, AggrEventInfo>,
}
impl Aggregator {
    pub(super) fn new() -> Self {
        Self { data: RMapVec::new() }
    }
    pub(super) fn add_entry(&mut self, start_delay: AttrVal, cycle: Cycle, output: Output<AttrVal>) {
        self.data.add_entry(
            AggrKey::new(&cycle, &output),
            AggrEventInfo {
                start_delay,
                cycle,
                output,
            },
        )
    }
    pub(super) fn into_sim_events(self, events: &mut BinaryHeap<CapSimEvent>) {
        for mut aggr_group in self.data.into_values() {
            Aggregator::process_aggr_group(&mut aggr_group, events, |l, r| l > r);
            Aggregator::process_aggr_group(&mut aggr_group, events, |l, r| l < r);
        }
    }
    fn process_aggr_group(
        aggr_group: &mut Vec<AggrEventInfo>,
        events: &mut BinaryHeap<CapSimEvent>,
        filter_fn: fn(AttrVal, AttrVal) -> bool,
    ) {
        events.extend(
            aggr_group
                .extract_if(.., |v| filter_fn(v.output.get_amount(), OF(0.0)))
                .reduce(|mut l, r| {
                    l.output.add_amount(r.output.get_amount());
                    l
                })
                .map(Into::into),
        );
    }
}

// Intermediate representation of event exists only to be able to aggregate data before it gets
// converted into cap sim events, where some data needed for aggregation will be lost
struct AggrEventInfo {
    start_delay: AttrVal,
    cycle: Cycle,
    output: Output<AttrVal>,
}
impl From<AggrEventInfo> for CapSimEvent {
    fn from(intermediate: AggrEventInfo) -> Self {
        CapSimEvent::CycleCheck(CapSimEventCycleCheck {
            time: intermediate.start_delay,
            cycle_iter: intermediate.cycle.iter_cycles(),
            output: intermediate.output,
        })
    }
}

// Aggregation key with rounded floats
#[derive(Eq, PartialEq, Hash)]
struct AggrKey {
    cycle: Cycle,
    output: AggrKeyOutput,
}
impl AggrKey {
    fn new(cycle: &Cycle, output: &Output<AttrVal>) -> Self {
        Self {
            cycle: cycle.copy_rounded(),
            output: output.into(),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
enum AggrKeyOutput {
    Simple(AggrKeyOutputSimple),
    Complex(AggrKeyOutputComplex),
}
impl From<&Output<AttrVal>> for AggrKeyOutput {
    fn from(output: &Output<AttrVal>) -> Self {
        match output {
            Output::Simple(simple) => AggrKeyOutput::Simple(simple.into()),
            Output::Complex(complex) => AggrKeyOutput::Complex(complex.into()),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct AggrKeyOutputSimple {
    delay: AttrVal,
}
impl From<&OutputSimple<AttrVal>> for AggrKeyOutputSimple {
    fn from(output: &OutputSimple<AttrVal>) -> Self {
        Self {
            delay: sig_round(output.delay, 10),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct AggrKeyOutputComplex {
    delay: AttrVal,
    repeats: Count,
    interval: AttrVal,
}
impl From<&OutputComplex<AttrVal>> for AggrKeyOutputComplex {
    fn from(output: &OutputComplex<AttrVal>) -> Self {
        Self {
            delay: sig_round(output.delay, 10),
            repeats: output.repeats,
            interval: sig_round(output.interval, 10),
        }
    }
}
