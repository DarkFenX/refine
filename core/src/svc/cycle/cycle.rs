use super::{
    cycle_infinite1::{CycleInfinite1, CycleInfinite1Iter},
    cycle_infinite2::{CycleInfinite2, CycleInfinite2Iter},
    cycle_infinite3::{CycleInfinite3, CycleInfinite3Iter},
    cycle_iter_item::CycleIterItem,
    cycle_limited::{CycleLimited, CycleLimitedIter},
    cycle_reload2::{CycleReload2, CycleReload2Iter},
};
use crate::{def::AttrVal, util::InfCount};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum Cycle {
    Limited(CycleLimited),
    Infinite1(CycleInfinite1),
    Infinite2(CycleInfinite2),
    Infinite3(CycleInfinite3),
    Reload2(CycleReload2),
}
impl Cycle {
    pub(in crate::svc) fn is_infinite(&self) -> bool {
        match &self {
            Self::Limited(_) => false,
            Self::Infinite1(_) | Self::Infinite2(_) | Self::Infinite3(_) | Self::Reload2(_) => true,
        }
    }
    pub(in crate::svc) fn get_charged_cycles(&self) -> InfCount {
        match self {
            Self::Limited(limited) => limited.get_charged_cycles(),
            Self::Infinite1(infinite1) => infinite1.get_charged_cycles(),
            Self::Infinite2(infinite2) => infinite2.get_charged_cycles(),
            Self::Infinite3(infinite3) => infinite3.get_charged_cycles(),
            Self::Reload2(reload2) => reload2.get_charged_cycles(),
        }
    }
    pub(in crate::svc) fn get_average_cycle_time(&self) -> AttrVal {
        match self {
            Self::Limited(limited) => limited.get_average_cycle_time(),
            Self::Infinite1(infinite1) => infinite1.get_average_cycle_time(),
            Self::Infinite2(infinite2) => infinite2.get_average_cycle_time(),
            Self::Infinite3(infinite3) => infinite3.get_average_cycle_time(),
            Self::Reload2(reload2) => reload2.get_average_cycle_time(),
        }
    }
    pub(in crate::svc) fn iter_cycles(&self) -> CycleIter {
        match self {
            Self::Limited(limited) => CycleIter::Limited(limited.iter_cycles()),
            Self::Infinite1(infinite1) => CycleIter::Infinite1(infinite1.iter_cycles()),
            Self::Infinite2(infinite2) => CycleIter::Infinite2(infinite2.iter_cycles()),
            Self::Infinite3(infinite3) => CycleIter::Infinite3(infinite3.iter_cycles()),
            Self::Reload2(reload2) => CycleIter::Reload2(reload2.iter_cycles()),
        }
    }
    // Methods used in cycle staggering
    pub(in crate::svc) fn copy_rounded(&self) -> Self {
        match self {
            Self::Limited(limited) => Self::Limited(limited.copy_rounded()),
            Self::Infinite1(infinite1) => Self::Infinite1(infinite1.copy_rounded()),
            Self::Infinite2(infinite2) => Self::Infinite2(infinite2.copy_rounded()),
            Self::Infinite3(infinite3) => Self::Infinite3(infinite3.copy_rounded()),
            Self::Reload2(reload2) => Self::Reload2(reload2.copy_rounded()),
        }
    }
    pub(in crate::svc) fn get_cycle_time_for_stagger(&self) -> AttrVal {
        match self {
            Self::Limited(limited) => limited.get_cycle_time_for_stagger(),
            Self::Infinite1(infinite1) => infinite1.get_cycle_time_for_stagger(),
            Self::Infinite2(infinite2) => infinite2.get_cycle_time_for_stagger(),
            Self::Infinite3(infinite3) => infinite3.get_cycle_time_for_stagger(),
            Self::Reload2(reload2) => reload2.get_cycle_time_for_stagger(),
        }
    }
}

pub(in crate::svc) enum CycleIter {
    Limited(CycleLimitedIter),
    Infinite1(CycleInfinite1Iter),
    Infinite2(CycleInfinite2Iter),
    Infinite3(CycleInfinite3Iter),
    Reload2(CycleReload2Iter),
}
impl Iterator for CycleIter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Limited(limited) => limited.next(),
            Self::Infinite1(infinite1) => infinite1.next(),
            Self::Infinite2(infinite2) => infinite2.next(),
            Self::Infinite3(infinite3) => infinite3.next(),
            Self::Reload2(reload2) => reload2.next(),
        }
    }
}
