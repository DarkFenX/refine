use std::collections::BinaryHeap;

use super::timing_key::{CSeqPartTimingKey, TIME_ROUND_DIGITS};
use crate::{
    num::PValue,
    svc::{
        cycle::CycleSeq,
        vast::{
            aggr::AggrIterData,
            stats::cap::sim::{
                event::{CapSimEvent, CapSimEventCycleCheck},
                shared::Direction,
            },
        },
    },
    util::RMapVec,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct MergeKeyRegular {
    start_delay: PValue,
    cseq: CycleSeq<CSeqPartTimingKey>,
}
impl MergeKeyRegular {
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

struct MergeEntry {
    start_delay: PValue,
    iter_data: AggrIterData<PValue>,
}

pub(super) struct Merger {
    mergeable_regular_gains: RMapVec<MergeKeyRegular, MergeEntry>,
    mergeable_regular_losses: RMapVec<MergeKeyRegular, MergeEntry>,
    gains: Vec<MergeEntry>,
    losses: Vec<MergeEntry>,
}
impl Merger {
    pub(super) fn new() -> Self {
        Self {
            mergeable_regular_gains: RMapVec::new(),
            mergeable_regular_losses: RMapVec::new(),
            gains: Vec::new(),
            losses: Vec::new(),
        }
    }
    pub(super) fn add_entry(&mut self, start_delay: PValue, iter_data: AggrIterData<PValue>, direction: Direction) {
        // Mergeable
        if let Some(regular_key) = MergeKeyRegular::try_new(start_delay, &iter_data) {
            let container = match direction {
                Direction::Gain => &mut self.mergeable_regular_gains,
                Direction::Loss => &mut self.mergeable_regular_losses,
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
        Merger::merge_regular(self.mergeable_regular_gains, &mut self.gains);
        Merger::merge_regular(self.mergeable_regular_losses, &mut self.losses);
        Merger::process_entries(self.gains, Direction::Gain, events);
        Merger::process_entries(self.losses, Direction::Loss, events);
    }
    fn merge_regular(mergeable: RMapVec<MergeKeyRegular, MergeEntry>, entries: &mut Vec<MergeEntry>) {
        for group_entries in mergeable.into_values() {
            entries.extend(group_entries);
        }
    }
    fn process_entries(merge_group: Vec<MergeEntry>, direction: Direction, events: &mut BinaryHeap<CapSimEvent>) {
        for entry in merge_group {
            events.push(CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                time: entry.start_delay,
                cycle_iter: entry.iter_data.iter(),
                direction,
            }))
        }
    }
}
