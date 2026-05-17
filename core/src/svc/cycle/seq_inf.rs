use crate::{
    svc::cycle::{CycleHardDt, CycleSeq, CycleSeqLooped},
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqInf<T> {
    pub(in crate::svc) data: T,
    // Optional hard downtime every cycle
    pub(in crate::svc) hard_dt: Option<CycleHardDt>,
}
impl<T> CSeqInf<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.data
    }
    pub(super) fn get_hard_dt(&self) -> Option<CycleHardDt> {
        self.hard_dt
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqInf<T> {
    pub(super) fn convert<U>(self) -> CSeqInf<U>
    where
        U: From<T>,
    {
        CSeqInf {
            data: self.data.into(),
            hard_dt: self.hard_dt,
        }
    }
    pub(in crate::svc) fn convert_with<C, U>(self, converter: &mut C) -> CSeqInf<U>
    where
        C: LibConverter<T, U>,
    {
        CSeqInf {
            data: converter.lib_convert(self.data),
            hard_dt: self.hard_dt,
        }
    }
    pub(in crate::svc) fn optimize(self) -> CycleSeq<T> {
        CycleSeq::Inf(self)
    }
    pub(super) fn optimize_looped(self) -> CycleSeqLooped<T> {
        CycleSeqLooped::Inf(self)
    }
}
impl<T> CSeqInf<T>
where
    T: Copy,
{
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::Inf(*self))
    }
}
