use std::collections::BinaryHeap;

use super::timing_key::{CSeqPartTimingKey, TIME_ROUND_DIGITS};
use crate::{
    num::PValue,
    svc::{
        cycle::{CSeqInf, CSeqLim, CSeqLimInf, CSeqLimSinInf, CSeqLoopLimSin, CycleSeq},
        output::{Output, OutputComplex, OutputSimple},
        vast::{
            aggr::{AggrIterData, AggrPartDataRegular},
            stats::cap::sim::{
                event::{CapSimEvent, CapSimEventCycleCheck},
                shared::Direction,
            },
        },
    },
    util::RMapVec,
};

pub(super) struct Merger {
    mergeable_gains: RMapVec<MergeKey, MergeEntry>,
    mergeable_losses: RMapVec<MergeKey, MergeEntry>,
    gains: Vec<MergeEntry>,
    losses: Vec<MergeEntry>,
}
impl Merger {
    pub(super) fn new() -> Self {
        Self {
            mergeable_gains: RMapVec::new(),
            mergeable_losses: RMapVec::new(),
            gains: Vec::new(),
            losses: Vec::new(),
        }
    }
    pub(super) fn add_entry(&mut self, start_delay: PValue, iter_data: AggrIterData<PValue>, direction: Direction) {
        // Mergeable
        if let Some(regular_key) = MergeKey::try_new(start_delay, &iter_data) {
            let container = match direction {
                Direction::Gain => &mut self.mergeable_gains,
                Direction::Loss => &mut self.mergeable_losses,
            };
            container.add_entry(regular_key, MergeEntry { start_delay, iter_data });
            return;
        }
        // Non-mergeable
        let container = match direction {
            Direction::Gain => &mut self.gains,
            Direction::Loss => &mut self.losses,
        };
        container.push(MergeEntry { start_delay, iter_data })
    }
    pub(super) fn into_sim_events(mut self, events: &mut BinaryHeap<CapSimEvent>) {
        Merger::merge(self.mergeable_gains, &mut self.gains);
        Merger::merge(self.mergeable_losses, &mut self.losses);
        Merger::convert(self.gains, Direction::Gain, events);
        Merger::convert(self.losses, Direction::Loss, events);
    }
    fn merge(mergeable: RMapVec<MergeKey, MergeEntry>, entries: &mut Vec<MergeEntry>) {
        for group_entries in mergeable.into_values() {
            if group_entries.len() < 2 {
                entries.extend(group_entries);
                continue;
            }
            let mut group_iter = group_entries.into_iter();
            let mut main_entry = group_iter.next().unwrap();
            for secondary_entry in group_iter {
                // Put secondary entry itself into target container if merging fails for some reason
                // (it really shouldn't, since the key should take care of checking match)
                if !main_entry.try_merge_instances(&secondary_entry) {
                    entries.push(secondary_entry);
                }
            }
            entries.push(main_entry);
        }
    }
    fn convert(merge_group: Vec<MergeEntry>, direction: Direction, events: &mut BinaryHeap<CapSimEvent>) {
        for entry in merge_group {
            events.push(CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                time: entry.start_delay,
                cycle_iter: entry.iter_data.iter(),
                direction,
            }))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Merge key
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct MergeKey {
    start_delay: PValue,
    cseq: CycleSeq<CSeqPartTimingKey>,
}
impl MergeKey {
    fn try_new(start_delay: PValue, iter_data: &AggrIterData<PValue>) -> Option<Self> {
        match iter_data {
            AggrIterData::Regular(_) => Some(Self {
                start_delay: start_delay.sig_rounded(TIME_ROUND_DIGITS),
                cseq: iter_data.extract_cseq_timing_key(),
            }),
            AggrIterData::Spool(_) => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Merge data
////////////////////////////////////////////////////////////////////////////////////////////////////
struct MergeEntry {
    start_delay: PValue,
    iter_data: AggrIterData<PValue>,
}
impl MergeEntry {
    fn try_merge_instances(&mut self, other: &Self) -> bool {
        match (&mut self.iter_data, &other.iter_data) {
            (AggrIterData::Regular(inner1), AggrIterData::Regular(inner2)) => {
                inner1.cseq.try_merge_instances(&inner2.cseq)
            }
            _ => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Necessary output impls
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Output<PValue> {
    fn increase_instance(&mut self, increase: PValue) {
        match self {
            Output::Simple(inner) => inner.increase_instance(increase),
            Output::Complex(inner) => inner.increase_instance(increase),
        }
    }
}
impl OutputSimple<PValue> {
    fn increase_instance(&mut self, increase: PValue) {
        self.instance += increase;
    }
}
impl OutputComplex<PValue> {
    fn increase_instance(&mut self, increase: PValue) {
        self.instance += increase;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Necessary cycle sequence impls
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CycleSeq<AggrPartDataRegular<PValue>> {
    fn try_merge_instances(&mut self, other: &Self) -> bool {
        match (self, other) {
            (CycleSeq::Lim(inner1), CycleSeq::Lim(inner2)) => {
                inner1.merge_instances(inner2);
                true
            }
            (CycleSeq::Inf(inner1), CycleSeq::Inf(inner2)) => {
                inner1.merge_instances(inner2);
                true
            }
            (CycleSeq::LimInf(inner1), CycleSeq::LimInf(inner2)) => {
                inner1.merge_instances(inner2);
                true
            }
            (CycleSeq::LimSinInf(inner1), CycleSeq::LimSinInf(inner2)) => {
                inner1.merge_instances(inner2);
                true
            }
            (CycleSeq::LoopLimSin(inner1), CycleSeq::LoopLimSin(inner2)) => {
                inner1.merge_instances(inner2);
                true
            }
            _ => false,
        }
    }
}
impl CSeqLim<AggrPartDataRegular<PValue>> {
    fn merge_instances(&mut self, other: &Self) {
        self.data.output.increase_instance(other.data.output.get_instance());
    }
}
impl CSeqInf<AggrPartDataRegular<PValue>> {
    fn merge_instances(&mut self, other: &Self) {
        self.data.output.increase_instance(other.data.output.get_instance());
    }
}
impl CSeqLimInf<AggrPartDataRegular<PValue>> {
    fn merge_instances(&mut self, other: &Self) {
        self.p1_data
            .output
            .increase_instance(other.p1_data.output.get_instance());
        self.p2_data
            .output
            .increase_instance(other.p2_data.output.get_instance());
    }
}
impl CSeqLimSinInf<AggrPartDataRegular<PValue>> {
    fn merge_instances(&mut self, other: &Self) {
        self.p1_data
            .output
            .increase_instance(other.p1_data.output.get_instance());
        self.p2_data
            .output
            .increase_instance(other.p2_data.output.get_instance());
        self.p3_data
            .output
            .increase_instance(other.p3_data.output.get_instance());
    }
}
impl CSeqLoopLimSin<AggrPartDataRegular<PValue>> {
    fn merge_instances(&mut self, other: &Self) {
        self.p1_data
            .output
            .increase_instance(other.p1_data.output.get_instance());
        self.p2_data
            .output
            .increase_instance(other.p2_data.output.get_instance());
    }
}
