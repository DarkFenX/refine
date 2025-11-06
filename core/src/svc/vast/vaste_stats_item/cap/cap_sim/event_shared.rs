use crate::{def::AttrVal, svc::cycle::CycleIter};

pub(super) struct CapSimEventCapGain {
    pub(super) time: AttrVal,
    pub(super) amount: AttrVal,
}

pub(super) struct CapSimEventInjectorAvailable {
    pub(super) time: AttrVal,
    pub(super) cycle_iter: CycleIter,
    pub(super) output: AttrVal,
}
