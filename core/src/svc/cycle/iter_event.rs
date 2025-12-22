use super::{
    cycle::Cycle, cycle_inf::CycleInfEventIter, cycle_lim::CycleLimEventIter, cycle_lim_inf::CycleLimInfEventIter,
    cycle_lim_sin_inf::CycleLimSinInfEventIter, cycle_loop_lim_sin::CycleLoopLimSinEventIter,
};
use crate::def::AttrVal;

impl Cycle {
    pub(in crate::svc) fn iter_events(&self) -> CycleEventIter {
        match self {
            Self::Lim(inner) => CycleEventIter::Lim(inner.iter_events()),
            Self::Inf(inner) => CycleEventIter::Inf(inner.iter_events()),
            Self::LimInf(inner) => CycleEventIter::LimInf(inner.iter_events()),
            Self::LimSinInf(inner) => CycleEventIter::LimSinInf(inner.iter_events()),
            Self::LoopLimSin(inner) => CycleEventIter::LoopLimSin(inner.iter_events()),
        }
    }
}

pub(in crate::svc) enum CycleEventIter {
    Lim(CycleLimEventIter),
    Inf(CycleInfEventIter),
    LimInf(CycleLimInfEventIter),
    LimSinInf(CycleLimSinInfEventIter),
    LoopLimSin(CycleLoopLimSinEventIter),
}
impl Iterator for CycleEventIter {
    type Item = CycleEventItem;

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

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleEventItem {
    // Time until next cycle starts
    pub(in crate::svc) time: AttrVal,
    // Is cycle sequence interrupted after this one or not
    pub(in crate::svc) interrupt: bool,
    // How charged current cycle is
    pub(in crate::svc) charged: Option<AttrVal>,
}
impl CycleEventItem {
    pub(super) fn new(time: AttrVal, interrupt: bool, charged: Option<AttrVal>) -> Self {
        Self {
            time,
            interrupt,
            charged,
        }
    }
}
