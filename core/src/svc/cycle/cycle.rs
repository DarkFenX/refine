use super::{
    cycle_inf::CycleInf, cycle_lim::CycleLim, cycle_lim_inf::CycleLimInf, cycle_lim_sin_inf::CycleLimSinInf,
    cycle_loop_lim_sin::CycleLoopLimSin,
};
use crate::{def::AttrVal, svc::cycle::CycleDataFull};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum Cycle<T = CycleDataFull> {
    Lim(CycleLim<T>),
    Inf(CycleInf<T>),
    LimInf(CycleLimInf<T>),
    LimSinInf(CycleLimSinInf<T>),
    LoopLimSin(CycleLoopLimSin<T>),
}
impl<T> Cycle<T>
where
    T: Copy,
{
    pub(in crate::svc) fn get_loop(&self) -> Option<CycleLooped<T>> {
        match self {
            Self::Lim(inner) => inner.get_loop(),
            Self::Inf(inner) => inner.get_loop(),
            Self::LimInf(inner) => inner.get_loop(),
            Self::LimSinInf(inner) => inner.get_loop(),
            Self::LoopLimSin(inner) => inner.get_loop(),
        }
    }
    pub(in crate::svc) fn get_first(&self) -> &T {
        match self {
            Self::Lim(inner) => inner.get_first(),
            Self::Inf(inner) => inner.get_first(),
            Self::LimInf(inner) => inner.get_first(),
            Self::LimSinInf(inner) => inner.get_first(),
            Self::LoopLimSin(inner) => inner.get_first(),
        }
    }
}
impl Cycle {
    pub(in crate::svc) fn get_average_time(&self) -> AttrVal {
        match self {
            Self::Lim(inner) => inner.get_average_time(),
            Self::Inf(inner) => inner.get_average_time(),
            Self::LimInf(inner) => inner.get_average_time(),
            Self::LimSinInf(inner) => inner.get_average_time(),
            Self::LoopLimSin(inner) => inner.get_average_time(),
        }
    }
    pub(in crate::svc) fn copy_rounded(&self) -> Self {
        match self {
            Self::Lim(inner) => Self::Lim(inner.copy_rounded()),
            Self::Inf(inner) => Self::Inf(inner.copy_rounded()),
            Self::LimInf(inner) => Self::LimInf(inner.copy_rounded()),
            Self::LimSinInf(inner) => Self::LimSinInf(inner.copy_rounded()),
            Self::LoopLimSin(inner) => Self::LoopLimSin(inner.copy_rounded()),
        }
    }
}

pub(in crate::svc) enum CycleLooped<T = CycleDataFull> {
    Inf(CycleInf<T>),
    LoopLimSin(CycleLoopLimSin<T>),
}
impl CycleLooped {
    pub(in crate::svc) fn get_average_time(&self) -> AttrVal {
        match self {
            Self::Inf(inner) => inner.get_average_time(),
            Self::LoopLimSin(inner) => inner.get_average_time(),
        }
    }
}
