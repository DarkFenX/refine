use super::{
    cycle_inf::CycleInfPartIter, cycle_lim::CycleLimPartIter, cycle_lim_inf::CycleLimInfPartIter,
    cycle_lim_sin_inf::CycleLimSinInfPartIter, cycle_loop_lim_sin::CycleLoopLimSinPartIter,
};
use crate::{svc::cycle::Cycle, util::InfCount};

pub(crate) struct CycleParts<'a, T> {
    cycle: &'a Cycle<T>,
    pub(crate) loops: bool,
}
impl<'a, T> CycleParts<'a, T>
where
    T: Copy,
{
    pub(crate) fn iter(&self) -> CyclePartIter<'a, T> {
        match self.cycle {
            Cycle::Lim(inner) => CyclePartIter::Lim(inner.iter_parts()),
            Cycle::Inf(inner) => CyclePartIter::Inf(inner.iter_parts()),
            Cycle::LimInf(inner) => CyclePartIter::LimInf(inner.iter_parts()),
            Cycle::LimSinInf(inner) => CyclePartIter::LimSinInf(inner.iter_parts()),
            Cycle::LoopLimSin(inner) => CyclePartIter::LoopLimSin(inner.iter_parts()),
        }
    }
}

pub(crate) struct CyclePart<T> {
    pub(crate) data: T,
    pub(crate) repeat_count: InfCount,
}

impl<T> Cycle<T>
where
    T: Copy,
{
    pub(in crate::svc) fn get_parts(&self) -> CycleParts<'_, T> {
        let loops = match self {
            Self::Lim(_) | Self::Inf(_) | Self::LimInf(_) | Self::LimSinInf(_) => false,
            Self::LoopLimSin(_) => true,
        };
        CycleParts { cycle: self, loops }
    }
}

pub(in crate::svc) enum CyclePartIter<'a, T> {
    Lim(CycleLimPartIter<'a, T>),
    Inf(CycleInfPartIter<'a, T>),
    LimInf(CycleLimInfPartIter<'a, T>),
    LimSinInf(CycleLimSinInfPartIter<'a, T>),
    LoopLimSin(CycleLoopLimSinPartIter<'a, T>),
}
impl<T> Iterator for CyclePartIter<'_, T>
where
    T: Copy,
{
    type Item = CyclePart<T>;

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
