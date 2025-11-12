use super::{
    cycle_reload1::{CycleReload1, CycleReload1Iter},
    cycle_reload2::{CycleReload2, CycleReload2Iter},
    cycle_simple::{CycleSimple, CycleSimpleIter},
};
use crate::{def::AttrVal, util::InfCount};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum Cycle {
    Simple(CycleSimple),
    Reload1(CycleReload1),
    Reload2(CycleReload2),
}
impl Cycle {
    pub(in crate::svc) fn is_infinite(&self) -> bool {
        match &self {
            Self::Simple(simple) => matches!(simple.repeat_count, InfCount::Infinite),
            Self::Reload1(_) => true,
            Self::Reload2(_) => true,
        }
    }
    pub(in crate::svc) fn get_cycles_until_empty(&self) -> InfCount {
        match self {
            Self::Simple(simple) => simple.get_cycles_until_empty(),
            Self::Reload1(reload1) => reload1.get_cycles_until_empty(),
            Self::Reload2(reload2) => reload2.get_cycles_until_empty(),
        }
    }
    pub(in crate::svc) fn get_average_cycle_time(&self) -> AttrVal {
        match self {
            Self::Simple(simple) => simple.get_average_cycle_time(),
            Self::Reload1(reload1) => reload1.get_average_cycle_time(),
            Self::Reload2(reload2) => reload2.get_average_cycle_time(),
        }
    }
    pub(in crate::svc) fn iter_cycles(&self) -> CycleIter {
        match self {
            Self::Simple(simple) => CycleIter::Simple(simple.iter_cycles()),
            Self::Reload1(reload1) => CycleIter::Reload1(reload1.iter_cycles()),
            Self::Reload2(reload2) => CycleIter::Reload2(reload2.iter_cycles()),
        }
    }
}

pub(in crate::svc) enum CycleIter {
    Simple(CycleSimpleIter),
    Reload1(CycleReload1Iter),
    Reload2(CycleReload2Iter),
}
impl Iterator for CycleIter {
    type Item = AttrVal;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Simple(simple) => simple.next(),
            Self::Reload1(reload1) => reload1.next(),
            Self::Reload2(reload2) => reload2.next(),
        }
    }
}
