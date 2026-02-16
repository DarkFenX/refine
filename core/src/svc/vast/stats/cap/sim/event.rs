use std::cmp::Ordering;

use crate::{
    num::PValue,
    svc::vast::{aggr::AggrIter, stats::cap::sim::shared::Direction},
    util::PrefetchPeekable,
};

pub(super) enum CapSimEvent {
    CycleCheck(CapSimEventCycleCheck),
    InjectorReady(CapSimEventInjector),
    CapChange(CapSimEventCapChange),
}
impl CapSimEvent {
    pub(super) fn get_time(&self) -> PValue {
        match self {
            Self::CycleCheck(event) => event.time,
            Self::InjectorReady(event) => event.time,
            Self::CapChange(event) => event.time,
        }
    }
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
        match other.get_time().cmp(&self.get_time()) {
            Ordering::Equal => match (self, other) {
                (Self::CycleCheck(_), Self::CycleCheck(_)) => Ordering::Equal,
                (Self::CycleCheck(_), _) => Ordering::Greater,
                (Self::InjectorReady(_), Self::InjectorReady(_)) => Ordering::Equal,
                (Self::InjectorReady(_), _) => Ordering::Greater,
                (Self::CapChange(e1), Self::CapChange(e2)) => match (e1.direction, e2.direction) {
                    (Direction::Gain, Direction::Gain) => e1.amount.cmp(&e2.amount),
                    (Direction::Loss, Direction::Loss) => e2.amount.cmp(&e1.amount),
                    (Direction::Gain, Direction::Loss) => Ordering::Greater,
                    (Direction::Loss, Direction::Gain) => Ordering::Less,
                },
                (Self::CapChange(_), _) => Ordering::Less,
            },
            result => result,
        }
    }
}
impl PartialEq<Self> for CapSimEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::CycleCheck(e1), Self::CycleCheck(e2)) => e1.time.eq(&e2.time),
            (Self::InjectorReady(e1), Self::InjectorReady(e2)) => e1.time.eq(&e2.time),
            (Self::CapChange(e1), Self::CapChange(e2)) => e1.time.eq(&e2.time) && e1.amount.eq(&e2.amount),
            _ => false,
        }
    }
}
impl Eq for CapSimEvent {}

pub(super) struct CapSimEventCycleCheck {
    pub(super) time: PValue,
    pub(super) cycle_iter: AggrIter<PValue>,
    pub(super) direction: Direction,
}

pub(super) struct CapSimEventCapChange {
    pub(super) time: PValue,
    pub(super) amount: PValue,
    pub(super) direction: Direction,
}

pub(super) struct CapSimEventInjector {
    pub(super) time: PValue,
    pub(super) cycle_iter: PrefetchPeekable<AggrIter<PValue>>,
}
impl CapSimEventInjector {
    pub(super) fn get_immediate_instance(&self) -> Option<PValue> {
        self.cycle_iter
            .peek()
            .and_then(|cycle_iter_item| cycle_iter_item.output.get_immediate_instance())
    }
}
