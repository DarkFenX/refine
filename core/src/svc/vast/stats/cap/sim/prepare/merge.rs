use std::collections::BinaryHeap;

use super::{output::OutputKey, shared::SIG_ROUND_DIGITS};
use crate::{
    num::{PValue, Value},
    svc::{
        cycle::{CycleDataDur, CycleDataDurCharge, CycleSeq},
        output::Output,
        vast::stats::cap::sim::event::{CapSimEvent, CapSimEventCycleCheck},
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
    pub(super) fn add_entry(&mut self, start_delay: PValue, cseq: CycleSeq<CycleDataDurCharge>, opc: Output<Value>) {
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
                .extract_if(.., |v| filter_fn(v.opc.get_instance(), Value::ZERO))
                .reduce(|mut l, r| {
                    l.opc.add_instance(r.opc.get_instance());
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
    cseq: CycleSeq<CycleDataDurCharge>,
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
    cseq: CycleSeq<CycleDataDur>,
    opc: OutputKey,
}
impl AggrKey {
    fn new(start_delay: PValue, cseq: &CycleSeq<CycleDataDurCharge>, opc: &Output<Value>) -> Self {
        Self {
            start_delay: start_delay.sig_rounded(SIG_ROUND_DIGITS),
            cseq: cseq.convert_and_optimize().copy_rounded(),
            opc: OutputKey::from_output(opc),
        }
    }
}
