use super::{
    cycle_infinite1::{CycleInfinite1, CycleInfinite1Iter},
    cycle_infinite2::{CycleInfinite2, CycleInfinite2Iter},
    cycle_infinite3::{CycleInfinite3, CycleInfinite3Iter},
    cycle_iter_item::CycleIterItem,
    cycle_limited::{CycleLimited, CycleLimitedIter},
    cycle_looped2::{CycleLooped2, CycleLooped2Iter},
};
use crate::{def::AttrVal, util::InfCount};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum Cycle {
    Limited(CycleLimited),
    Infinite1(CycleInfinite1),
    Infinite2(CycleInfinite2),
    Infinite3(CycleInfinite3),
    Looped2(CycleLooped2),
}
impl Cycle {
    pub(in crate::svc) fn is_infinite(&self) -> bool {
        match &self {
            Self::Limited(_) => false,
            Self::Infinite1(_) | Self::Infinite2(_) | Self::Infinite3(_) | Self::Looped2(_) => true,
        }
    }
    pub(in crate::svc) fn get_charged_cycles(&self) -> InfCount {
        match self {
            Self::Limited(inner) => inner.get_charged_cycles(),
            Self::Infinite1(inner) => inner.get_charged_cycles(),
            Self::Infinite2(inner) => inner.get_charged_cycles(),
            Self::Infinite3(inner) => inner.get_charged_cycles(),
            Self::Looped2(inner) => inner.get_charged_cycles(),
        }
    }
    pub(in crate::svc) fn get_average_cycle_time(&self) -> AttrVal {
        match self {
            Self::Limited(inner) => inner.get_average_cycle_time(),
            Self::Infinite1(inner) => inner.get_average_cycle_time(),
            Self::Infinite2(inner) => inner.get_average_cycle_time(),
            Self::Infinite3(inner) => inner.get_average_cycle_time(),
            Self::Looped2(inner) => inner.get_average_cycle_time(),
        }
    }
    pub(in crate::svc) fn iter_cycles(&self) -> CycleIter {
        match self {
            Self::Limited(inner) => CycleIter::Limited(inner.iter_cycles()),
            Self::Infinite1(inner) => CycleIter::Infinite1(inner.iter_cycles()),
            Self::Infinite2(inner) => CycleIter::Infinite2(inner.iter_cycles()),
            Self::Infinite3(inner) => CycleIter::Infinite3(inner.iter_cycles()),
            Self::Looped2(inner) => CycleIter::Looped2(inner.iter_cycles()),
        }
    }
    // Methods used in cycle staggering
    pub(in crate::svc) fn copy_rounded(&self) -> Self {
        match self {
            Self::Limited(inner) => Self::Limited(inner.copy_rounded()),
            Self::Infinite1(inner) => Self::Infinite1(inner.copy_rounded()),
            Self::Infinite2(inner) => Self::Infinite2(inner.copy_rounded()),
            Self::Infinite3(inner) => Self::Infinite3(inner.copy_rounded()),
            Self::Looped2(inner) => Self::Looped2(inner.copy_rounded()),
        }
    }
    pub(in crate::svc) fn get_cycle_time_for_stagger(&self) -> AttrVal {
        match self {
            Self::Limited(inner) => inner.get_cycle_time_for_stagger(),
            Self::Infinite1(inner) => inner.get_cycle_time_for_stagger(),
            Self::Infinite2(inner) => inner.get_cycle_time_for_stagger(),
            Self::Infinite3(inner) => inner.get_cycle_time_for_stagger(),
            Self::Looped2(inner) => inner.get_cycle_time_for_stagger(),
        }
    }
}

pub(in crate::svc) enum CycleIter {
    Limited(CycleLimitedIter),
    Infinite1(CycleInfinite1Iter),
    Infinite2(CycleInfinite2Iter),
    Infinite3(CycleInfinite3Iter),
    Looped2(CycleLooped2Iter),
}
impl Iterator for CycleIter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Limited(inner) => inner.next(),
            Self::Infinite1(inner) => inner.next(),
            Self::Infinite2(inner) => inner.next(),
            Self::Infinite3(inner) => inner.next(),
            Self::Looped2(inner) => inner.next(),
        }
    }
}
