use super::{
    cycle_inf::{CycleInfPartIter, CycleLoopedInfPartIter},
    cycle_lim::CycleLimPartIter,
    cycle_lim_inf::CycleLimInfPartIter,
    cycle_lim_sin_inf::CycleLimSinInfPartIter,
    cycle_loop_lim_sin::{CycleLoopLimSinPartIter, CycleLoopedLoopLimSinPartIter},
};
use crate::{
    def::Count,
    svc::cycle::{Cycle, CycleLooped},
    util::InfCount,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular cycle
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct CyclePart<T> {
    pub(crate) data: T,
    pub(crate) repeat_count: InfCount,
}

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
            Cycle::Lim(inner) => CyclePartIter::Lim(inner.iter_parts_regular()),
            Cycle::Inf(inner) => CyclePartIter::Inf(inner.iter_parts_regular()),
            Cycle::LimInf(inner) => CyclePartIter::LimInf(inner.iter_parts_regular()),
            Cycle::LimSinInf(inner) => CyclePartIter::LimSinInf(inner.iter_parts_regular()),
            Cycle::LoopLimSin(inner) => CyclePartIter::LoopLimSin(inner.iter_parts_regular()),
        }
    }
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Looped cycle
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct CycleLoopedPart<T> {
    pub(crate) data: T,
    pub(crate) repeat_count: Count,
}

impl<T> CycleLooped<T>
where
    T: Copy,
{
    pub(in crate::svc) fn iter_parts(&self) -> CycleLoopedPartIter<'_, T> {
        match self {
            Self::Inf(inner) => CycleLoopedPartIter::Inf(inner.iter_parts_looped()),
            Self::LoopLimSin(inner) => CycleLoopedPartIter::LoopLimSin(inner.iter_parts_looped()),
        }
    }
}

pub(in crate::svc) enum CycleLoopedPartIter<'a, T> {
    Inf(CycleLoopedInfPartIter<'a, T>),
    LoopLimSin(CycleLoopedLoopLimSinPartIter<'a, T>),
}
impl<T> Iterator for CycleLoopedPartIter<'_, T>
where
    T: Copy,
{
    type Item = CycleLoopedPart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Inf(inner) => inner.next(),
            Self::LoopLimSin(inner) => inner.next(),
        }
    }
}
