use std::collections::BinaryHeap;

use super::shared::Direction;
use crate::{
    num::PValue,
    svc::vast::{aggr::AggrIterData, stats::cap::sim::event::CapSimEvent},
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
        for mut aggr_group in self.data.into_values() {
            Aggregator::process_aggr_group(&mut aggr_group, events, |l, r| l > r);
            Aggregator::process_aggr_group(&mut aggr_group, events, |l, r| l < r);
        }
    }
}
