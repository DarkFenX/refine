use std::cmp::Ordering;

use crate::{
    def::AttrVal,
    svc::{
        cycle::{CycleDataTimeChargedness, CycleEventIter},
        output::Output,
    },
};

pub(super) enum CapSimEvent {
    CycleCheck(CapSimEventCycleCheck),
    InjectorReady(CapSimEventInjector),
    CapGain(CapSimEventCapGain),
}
impl CapSimEvent {
    pub(super) fn get_time(&self) -> AttrVal {
        match self {
            Self::CycleCheck(event) => event.time,
            Self::InjectorReady(event) => event.time,
            Self::CapGain(event) => event.time,
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
                (Self::CapGain(e1), Self::CapGain(e2)) => e1.amount.cmp(&e2.amount),
                (Self::CapGain(_), _) => Ordering::Less,
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
            (Self::CapGain(e1), Self::CapGain(e2)) => e1.time.eq(&e2.time) && e1.amount.eq(&e2.amount),
            _ => false,
        }
    }
}
impl Eq for CapSimEvent {}

pub(super) struct CapSimEventCycleCheck {
    pub(super) time: AttrVal,
    pub(super) cycle_iter: CycleEventIter<CycleDataTimeChargedness>,
    pub(super) output: Output<AttrVal>,
}

pub(super) struct CapSimEventCapGain {
    pub(super) time: AttrVal,
    pub(super) amount: AttrVal,
}

pub(super) struct CapSimEventInjector {
    pub(super) time: AttrVal,
    pub(super) cycle_iter: CycleEventIter<CycleDataTimeChargedness>,
    pub(super) output: AttrVal,
}
