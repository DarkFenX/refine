use super::{cycle_reload1::CycleReload1, cycle_reload2::CycleReload2, cycle_simple::CycleSimple};
use crate::{def::AttrVal, util::InfCount};

#[derive(Copy, Clone)]
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
}
