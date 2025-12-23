use super::{
    cycle_inf::CycleInfEventIter, cycle_lim::CycleLimEventIter, cycle_lim_inf::CycleLimInfEventIter,
    cycle_lim_sin_inf::CycleLimSinInfEventIter, cycle_loop_lim_sin::CycleLoopLimSinEventIter,
};
use crate::svc::cycle::Cycle;

impl<T> Cycle<T>
where
    T: Copy,
{
    pub(in crate::svc) fn iter_events(&self) -> CycleEventIter<T> {
        match self {
            Self::Lim(inner) => CycleEventIter::Lim(inner.iter_events()),
            Self::Inf(inner) => CycleEventIter::Inf(inner.iter_events()),
            Self::LimInf(inner) => CycleEventIter::LimInf(inner.iter_events()),
            Self::LimSinInf(inner) => CycleEventIter::LimSinInf(inner.iter_events()),
            Self::LoopLimSin(inner) => CycleEventIter::LoopLimSin(inner.iter_events()),
        }
    }
}

pub(in crate::svc) enum CycleEventIter<T> {
    Lim(CycleLimEventIter<T>),
    Inf(CycleInfEventIter<T>),
    LimInf(CycleLimInfEventIter<T>),
    LimSinInf(CycleLimSinInfEventIter<T>),
    LoopLimSin(CycleLoopLimSinEventIter<T>),
}
impl<T> Iterator for CycleEventIter<T>
where
    T: Copy,
{
    type Item = T;

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
