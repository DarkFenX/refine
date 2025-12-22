use super::{
    cycle_inf::{CycleInf, CycleInfIter},
    cycle_lim::{CycleLim, CycleLimIter},
    cycle_lim_inf::{CycleLimInf, CycleLimInfIter},
    cycle_lim_sin_inf::{CycleLimSinInf, CycleLimSinInfIter},
    cycle_loop_lim_sin::{CycleLoopLimSin, CycleLoopLimSinIter},
};
use crate::{
    def::AttrVal,
    svc::cycle::{CycleChargedInfo, CycleIterItem},
    util::InfCount,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum Cycle {
    Lim(CycleLim),
    Inf(CycleInf),
    LimInf(CycleLimInf),
    LimSinInf(CycleLimSinInf),
    LoopLimSin(CycleLoopLimSin),
}
impl Cycle {
    pub(in crate::svc) fn get_looped_part(&self) -> Option<CycleLooped> {
        match self {
            Self::Lim(inner) => inner.get_looped_part(),
            Self::Inf(inner) => inner.get_looped_part(),
            Self::LimInf(inner) => inner.get_looped_part(),
            Self::LimSinInf(inner) => inner.get_looped_part(),
            Self::LoopLimSin(inner) => inner.get_looped_part(),
        }
    }
    pub(in crate::svc) fn get_charged_info(&self) -> InfCount {
        match self {
            Self::Lim(inner) => inner.get_charged_info(),
            Self::Inf(inner) => inner.get_charged_info(),
            Self::LimInf(inner) => inner.get_charged_info(),
            Self::LimSinInf(inner) => inner.get_charged_info(),
            Self::LoopLimSin(inner) => inner.get_charged_info(),
        }
    }
    pub(in crate::svc) fn get_average_cycle_time(&self) -> AttrVal {
        match self {
            Self::Lim(inner) => inner.get_average_cycle_time(),
            Self::Inf(inner) => inner.get_average_cycle_time(),
            Self::LimInf(inner) => inner.get_average_cycle_time(),
            Self::LimSinInf(inner) => inner.get_average_cycle_time(),
            Self::LoopLimSin(inner) => inner.get_average_cycle_time(),
        }
    }
    pub(in crate::svc) fn iter_cycles(&self) -> CycleIter {
        match self {
            Self::Lim(inner) => CycleIter::Lim(inner.iter_cycles()),
            Self::Inf(inner) => CycleIter::Inf(inner.iter_cycles()),
            Self::LimInf(inner) => CycleIter::LimInf(inner.iter_cycles()),
            Self::LimSinInf(inner) => CycleIter::LimSinInf(inner.iter_cycles()),
            Self::LoopLimSin(inner) => CycleIter::LoopLimSin(inner.iter_cycles()),
        }
    }
    // Methods used in cycle staggering
    pub(in crate::svc) fn copy_rounded(&self) -> Self {
        match self {
            Self::Lim(inner) => Self::Lim(inner.copy_rounded()),
            Self::Inf(inner) => Self::Inf(inner.copy_rounded()),
            Self::LimInf(inner) => Self::LimInf(inner.copy_rounded()),
            Self::LimSinInf(inner) => Self::LimSinInf(inner.copy_rounded()),
            Self::LoopLimSin(inner) => Self::LoopLimSin(inner.copy_rounded()),
        }
    }
    pub(in crate::svc) fn get_first_cycle_time(&self) -> AttrVal {
        match self {
            Self::Lim(inner) => inner.get_first_cycle_time(),
            Self::Inf(inner) => inner.get_first_cycle_time(),
            Self::LimInf(inner) => inner.get_first_cycle_time(),
            Self::LimSinInf(inner) => inner.get_first_cycle_time(),
            Self::LoopLimSin(inner) => inner.get_first_cycle_time(),
        }
    }
}

pub(in crate::svc) enum CycleLooped {
    Inf(CycleInf),
    LoopLimSin(CycleLoopLimSin),
}
impl CycleLooped {
    pub(in crate::svc) fn get_average_cycle_time(&self) -> AttrVal {
        match self {
            Self::Inf(inner) => inner.get_average_cycle_time(),
            Self::LoopLimSin(inner) => inner.get_average_cycle_time(),
        }
    }
}

pub(in crate::svc) enum CycleIter {
    Lim(CycleLimIter),
    Inf(CycleInfIter),
    LimInf(CycleLimInfIter),
    LimSinInf(CycleLimSinInfIter),
    LoopLimSin(CycleLoopLimSinIter),
}
impl Iterator for CycleIter {
    type Item = CycleIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Lim(inner) => inner.next(),
            Self::Inf(inner) => inner.next(),
            Self::LimInf(inner) => inner.next(),
            Self::LimSinInf(inner) => inner.next(),
            Self::LoopLimSin(inner) => inner.next(),
        }
    }
}
