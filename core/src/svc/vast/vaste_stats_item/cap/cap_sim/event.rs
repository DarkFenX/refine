use std::cmp::Ordering;

use crate::{
    def::AttrVal,
    svc::{cycle::CycleIter, output::Output},
};

pub(super) enum CapSimEvent {
    // Next event time, amount
    CapChange(AttrVal, AttrVal),
    // Next event time, iterator, output
    Cycle(AttrVal, CycleIter, Output<AttrVal>),
}
impl CapSimEvent {
    pub(super) fn get_time(&self) -> AttrVal {
        match self {
            Self::CapChange(time, _) => *time,
            Self::Cycle(time, _, _) => *time,
        }
    }
    pub(super) fn get_amount(&self) -> Option<AttrVal> {
        match self {
            Self::CapChange(_, amount) => Some(*amount),
            Self::Cycle(_, _, _) => None,
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
        // - with equal time, cycle events are processed first, then cap change events amount desc
        match other.get_time().cmp(&self.get_time()) {
            Ordering::Equal => match (self.get_amount(), other.get_amount()) {
                (Some(s), Some(o)) => s.cmp(&o),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            result => result,
        }
    }
}
impl PartialEq<Self> for CapSimEvent {
    fn eq(&self, other: &Self) -> bool {
        self.get_time() == other.get_time() && self.get_amount() == other.get_amount()
    }
}
impl Eq for CapSimEvent {}
