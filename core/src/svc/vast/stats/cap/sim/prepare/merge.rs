use std::collections::BinaryHeap;

use crate::{
    num::PValue,
    svc::vast::{
        aggr::AggrIterData,
        stats::cap::sim::{
            event::{CapSimEvent, CapSimEventCycleCheck},
            shared::Direction,
        },
    },
};

struct MergeEntry {
    start_delay: PValue,
    iter_data: AggrIterData<PValue>,
}

pub(super) struct Merger {
    gains: Vec<MergeEntry>,
    losses: Vec<MergeEntry>,
}
impl Merger {
    pub(super) fn new() -> Self {
        Self {
            gains: Vec::new(),
            losses: Vec::new(),
        }
    }
    pub(super) fn add_entry(&mut self, start_delay: PValue, iter_data: AggrIterData<PValue>, direction: Direction) {
        let container = match direction {
            Direction::Gain => &mut self.gains,
            Direction::Loss => &mut self.losses,
        };
        container.push(MergeEntry { start_delay, iter_data })
    }
    pub(super) fn into_sim_events(self, events: &mut BinaryHeap<CapSimEvent>) {
        Merger::process_merge_group(self.gains, Direction::Gain, events);
        Merger::process_merge_group(self.losses, Direction::Loss, events);
    }
    fn process_merge_group(merge_group: Vec<MergeEntry>, direction: Direction, events: &mut BinaryHeap<CapSimEvent>) {
        for entry in merge_group {
            events.push(CapSimEvent::CycleCheck(CapSimEventCycleCheck {
                time: entry.start_delay,
                cycle_iter: entry.iter_data.iter(),
                direction,
            }))
        }
    }
}
