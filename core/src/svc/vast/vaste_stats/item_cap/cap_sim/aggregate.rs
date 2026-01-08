use std::collections::BinaryHeap;

use super::{
    event::{CapSimEvent, CapSimEventCycleCheck},
    shared::SIG_ROUND_DIGITS,
};
use crate::{
    misc::{Count, PValue, Value},
    svc::{
        cycle::{CycleDataTime, CycleDataTimeCharge, CycleSeq},
        output::{Output, OutputComplex, OutputSimple},
    },
    util::RMapVec,
};

pub(super) struct Aggregator {
    data: RMapVec<AggrKey, AggrEventInfo>,
}
impl Aggregator {
    pub(super) fn new() -> Self {
        Self { data: RMapVec::new() }
    }
    pub(super) fn add_entry(&mut self, start_delay: PValue, cseq: CycleSeq<CycleDataTimeCharge>, opc: Output<Value>) {
        self.data.add_entry(
            AggrKey::new(start_delay, &cseq, &opc),
            AggrEventInfo { start_delay, cseq, opc },
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
        filter_fn: fn(Value, Value) -> bool,
    ) {
        // TODO: check if get_amount() is the right method to use here
        events.extend(
            aggr_group
                .extract_if(.., |v| filter_fn(v.opc.get_amount(), Value::ZERO))
                .reduce(|mut l, r| {
                    l.opc.add_amount(r.opc.get_amount());
                    l
                })
                .map(AggrEventInfo::into_cap_sim_event),
        );
    }
}

// Intermediate representation of event exists only to be able to aggregate data before it gets
// converted into cap sim events, where some data needed for aggregation will be lost
struct AggrEventInfo {
    start_delay: PValue,
    cseq: CycleSeq<CycleDataTimeCharge>,
    opc: Output<Value>,
}
impl AggrEventInfo {
    fn into_cap_sim_event(self) -> CapSimEvent {
        CapSimEvent::CycleCheck(CapSimEventCycleCheck {
            time: self.start_delay,
            cycle_iter: self.cseq.iter_cycles(),
            opc: self.opc,
        })
    }
}

// Aggregation key with rounded floats
#[derive(Eq, PartialEq, Hash)]
struct AggrKey {
    start_delay: PValue,
    cseq: CycleSeq<CycleDataTime>,
    opc: AggrKeyOutput,
}
impl AggrKey {
    fn new(start_delay: PValue, cseq: &CycleSeq<CycleDataTimeCharge>, opc: &Output<Value>) -> Self {
        Self {
            start_delay: start_delay.sig_rounded(SIG_ROUND_DIGITS),
            cseq: cseq.convert().copy_rounded(),
            opc: AggrKeyOutput::from_output(opc),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
enum AggrKeyOutput {
    Simple(AggrKeyOutputSimple),
    Complex(AggrKeyOutputComplex),
}
impl AggrKeyOutput {
    fn from_output(output: &Output<Value>) -> Self {
        match output {
            Output::Simple(inner) => AggrKeyOutput::Simple(AggrKeyOutputSimple::from_output_simple(inner)),
            Output::Complex(inner) => AggrKeyOutput::Complex(AggrKeyOutputComplex::from_output_complex(inner)),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct AggrKeyOutputSimple {
    delay: PValue,
}
impl AggrKeyOutputSimple {
    fn from_output_simple(output_simple: &OutputSimple<Value>) -> Self {
        Self {
            delay: output_simple.delay.sig_rounded(SIG_ROUND_DIGITS),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct AggrKeyOutputComplex {
    delay: PValue,
    repeats: Count,
    interval: PValue,
}
impl AggrKeyOutputComplex {
    fn from_output_complex(output_complex: &OutputComplex<Value>) -> Self {
        Self {
            delay: output_complex.delay.sig_rounded(SIG_ROUND_DIGITS),
            repeats: output_complex.repeats,
            interval: output_complex.interval.sig_rounded(SIG_ROUND_DIGITS),
        }
    }
}
