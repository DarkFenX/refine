use std::cmp::Ordering;

use crate::{
    PValue,
    svc::vast::{aggr::AggrIter, stats::cap::sim::shared::Direction},
    util::PrefetchPeekable,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event main type
////////////////////////////////////////////////////////////////////////////////////////////////////
// Time is moved out of boxes to allow faster access
pub(super) struct CapSimEvent {
    pub(super) time: PValue,
    pub(super) data: CapSimEventData,
}
impl PartialOrd for CapSimEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CapSimEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Since sim is using max-heap, adjust parameters so that:
        // - events which have lower time are processed earlier
        // - with equal time, order of processing:
        //   - cycle check events
        //   - injector ready events
        //   - cap gain events, from highest to lowest
        match other.time.cmp(&self.time) {
            Ordering::Equal => match (&self.data, &other.data) {
                (CapSimEventData::CycleCheck(..), CapSimEventData::CycleCheck(..)) => Ordering::Equal,
                (CapSimEventData::CycleCheck(..), _) => Ordering::Greater,
                (CapSimEventData::InjectorReady(..), CapSimEventData::InjectorReady(..)) => Ordering::Equal,
                (CapSimEventData::InjectorReady(..), _) => Ordering::Greater,
                (CapSimEventData::CapChange(e1), CapSimEventData::CapChange(e2)) => {
                    match (e1.direction, e2.direction) {
                        (Direction::Gain, Direction::Gain) => e1.amount.cmp(&e2.amount),
                        (Direction::Loss, Direction::Loss) => e2.amount.cmp(&e1.amount),
                        (Direction::Gain, Direction::Loss) => Ordering::Greater,
                        (Direction::Loss, Direction::Gain) => Ordering::Less,
                    }
                }
                (CapSimEventData::CapChange(..), _) => Ordering::Less,
            },
            result => result,
        }
    }
}
impl PartialEq<Self> for CapSimEvent {
    fn eq(&self, other: &Self) -> bool {
        if !self.time.eq(&other.time) {
            return false;
        }
        match (&self.data, &other.data) {
            (CapSimEventData::CycleCheck(..), CapSimEventData::CycleCheck(..)) => true,
            (CapSimEventData::InjectorReady(..), CapSimEventData::InjectorReady(..)) => true,
            (CapSimEventData::CapChange(e1), CapSimEventData::CapChange(e2)) => e1.amount.eq(&e2.amount),
            _ => false,
        }
    }
}
impl Eq for CapSimEvent {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event enum and its variants
////////////////////////////////////////////////////////////////////////////////////////////////////
// Boxes to minimize type size, to allow binary heap move it faster
pub(super) enum CapSimEventData {
    CycleCheck(Box<CapSimEventCycleCheck>),
    InjectorReady(Box<CapSimEventInjector>),
    CapChange(CapSimEventCapChange),
}

pub(super) struct CapSimEventCycleCheck {
    pub(super) cycle_iter: AggrIter<PValue>,
    pub(super) direction: Direction,
}

pub(super) struct CapSimEventInjector {
    pub(super) cycle_iter: PrefetchPeekable<AggrIter<PValue>>,
}
impl CapSimEventInjector {
    pub(super) fn get_immediate_instance(&self) -> Option<PValue> {
        self.cycle_iter
            .peek()
            .and_then(|cycle_iter_item| cycle_iter_item.output.get_immediate_instance())
    }
}

pub(super) struct CapSimEventCapChange {
    pub(super) amount: PValue,
    pub(super) direction: Direction,
}
