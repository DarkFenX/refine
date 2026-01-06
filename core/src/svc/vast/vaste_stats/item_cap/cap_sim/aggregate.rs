use std::collections::BinaryHeap;

use super::event::{CapSimEvent, CapSimEventCycleCheck};
use crate::{
    def::{AttrVal, DefCount, OF},
    svc::{
        cycle::{CycleDataTime, CycleDataTimeCharge, CycleSeq},
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
    pub(super) fn add_entry(
        &mut self,
        start_delay: AttrVal,
        cseq: CycleSeq<CycleDataTimeCharge>,
        opc: Output<AttrVal>,
    ) {
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
        filter_fn: fn(AttrVal, AttrVal) -> bool,
    ) {
        // TODO: check if get_amount() is the right method to use here
        events.extend(
            aggr_group
                .extract_if(.., |v| filter_fn(v.opc.get_amount(), OF(0.0)))
                .reduce(|mut l, r| {
                    l.opc.add_amount(r.opc.get_amount());
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
    cseq: CycleSeq<CycleDataTimeCharge>,
    opc: Output<AttrVal>,
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
    start_delay: AttrVal,
    cseq: CycleSeq<CycleDataTime>,
    opc: AggrKeyOutput,
}
impl AggrKey {
    fn new(start_delay: AttrVal, cseq: &CycleSeq<CycleDataTimeCharge>, opc: &Output<AttrVal>) -> Self {
        Self {
            start_delay: sig_round(start_delay, 10),
            cseq: cseq.convert().copy_rounded(),
            opc: opc.into(),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
enum AggrKeyOutput {
    Simple(AggrKeyOutputSimple),
    Complex(AggrKeyOutputComplex),
}
impl AggrKeyOutput {
    fn from_output(output: &Output<AttrVal>) -> Self {
        match output {
            Output::Simple(inner) => AggrKeyOutput::Simple(inner.into()),
            Output::Complex(inner) => AggrKeyOutput::Complex(inner.into()),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct AggrKeyOutputSimple {
    delay: AttrVal,
}
impl AggrKeyOutputSimple {
    fn from_output_simple(output_simple: &OutputSimple<AttrVal>) -> Self {
        Self {
            delay: sig_round(output_simple.delay, 10),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct AggrKeyOutputComplex {
    delay: AttrVal,
    repeats: DefCount,
    interval: AttrVal,
}
impl AggrKeyOutputComplex {
    fn from_output_complex(output_complex: &OutputComplex<AttrVal>) -> Self {
        Self {
            delay: sig_round(output_complex.delay, 10),
            repeats: output_complex.repeats,
            interval: sig_round(output_complex.interval, 10),
        }
    }
}
