use super::{
    cycle_inf::CycleInf, cycle_lim::CycleLim, cycle_lim_inf::CycleLimInf, cycle_lim_sin_inf::CycleLimSinInf,
    cycle_loop_lim_sin::CycleLoopLimSin,
};
use crate::{def::AttrVal, svc::cycle::CycleChargedInfoIter};

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
    pub(in crate::svc) fn iter_charged_info(&self) -> CycleChargedInfoIter {
        match self {
            Self::Lim(inner) => inner.iter_charged_info(),
            Self::Inf(inner) => inner.iter_charged_info(),
            Self::LimInf(inner) => inner.iter_charged_info(),
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
