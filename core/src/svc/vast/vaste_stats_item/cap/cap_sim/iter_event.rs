use std::cmp::Ordering;

use crate::{
    def::AttrVal,
    svc::{cycle::CycleIter, output::Output},
};

pub(super) enum CapSimIterEvent {
    Cycle(CapSimIterEventCycle),
    InjectorAvailable(CapSimIterEventInjectorAvailable),
    CapGain(CapSimIterEventCapGain),
}
impl CapSimIterEvent {
    pub(super) fn get_time(&self) -> AttrVal {
        match self {
            Self::CapGain(event) => event.time,
            Self::Cycle(event) => event.time,
            Self::InjectorAvailable(event) => event.time,
        }
    }
}
impl PartialOrd for CapSimIterEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CapSimIterEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Since sim is using max-heap, adjust parameters so that:
        // - events which have lower time are processed earlier
        // - with equal time, order of processing:
        //   - cycle events
        //   - injector available events
        //   - cap gain events, from highest to lowest
        match other.get_time().cmp(&self.get_time()) {
            Ordering::Equal => match (self, other) {
                (Self::Cycle(_), Self::Cycle(_)) => Ordering::Equal,
                (Self::Cycle(_), _) => Ordering::Greater,
                (Self::InjectorAvailable(_), Self::InjectorAvailable(_)) => Ordering::Equal,
                (Self::InjectorAvailable(_), _) => Ordering::Greater,
                (Self::CapGain(e1), Self::CapGain(e2)) => e1.amount.cmp(&e2.amount),
                (Self::CapGain(_), _) => Ordering::Less,
            },
            result => result,
        }
    }
}
impl PartialEq<Self> for CapSimIterEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Cycle(e1), Self::Cycle(e2)) => e1.time.eq(&e2.time),
            (Self::InjectorAvailable(e1), Self::InjectorAvailable(e2)) => e1.time.eq(&e2.time),
            (Self::CapGain(e1), Self::CapGain(e2)) => e1.time.eq(&e2.time) && e1.amount.eq(&e2.amount),
            _ => false,
        }
    }
}
impl Eq for CapSimIterEvent {}

pub(super) struct CapSimIterEventCycle {
    pub(super) time: AttrVal,
    pub(super) cycle_iter: CycleIter,
    pub(super) output: Output<AttrVal>,
}

pub(super) struct CapSimIterEventInjectorAvailable {
    pub(super) time: AttrVal,
    pub(super) cycle_iter: CycleIter,
    pub(super) output: AttrVal,
}

pub(super) struct CapSimIterEventCapGain {
    pub(super) time: AttrVal,
    pub(super) amount: AttrVal,
}
