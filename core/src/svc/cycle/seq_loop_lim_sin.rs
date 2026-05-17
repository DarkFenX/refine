use crate::{
    num::Count,
    svc::cycle::{CycleHardDt, CycleSeq, CycleSeqLooped, seq_inf::CSeqInf},
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Following parts are lopped:
// Part 1: runs specified number of times
// Part 2: runs once
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLoopLimSin<T> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
    // Optional hard downtime every loop
    pub(in crate::svc) hard_dt: Option<CycleHardDt>,
}
impl<T> CSeqLoopLimSin<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn get_hard_dt(&self) -> Option<CycleHardDt> {
        self.hard_dt
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqLoopLimSin<T> {
    pub(super) fn convert<U>(self) -> CSeqLoopLimSin<U>
    where
        U: From<T>,
    {
        CSeqLoopLimSin {
            p1_data: U::from(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: U::from(self.p2_data),
            hard_dt: self.hard_dt,
        }
    }
    pub(in crate::svc) fn convert_with<C, U>(self, converter: &mut C) -> CSeqLoopLimSin<U>
    where
        C: LibConverter<T, U>,
    {
        CSeqLoopLimSin {
            p1_data: converter.lib_convert(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: converter.lib_convert(self.p2_data),
            hard_dt: self.hard_dt,
        }
    }
    pub(in crate::svc) fn optimize(self) -> CycleSeq<T>
    where
        T: Eq,
    {
        match self.p1_data == self.p2_data && self.hard_dt.is_none() {
            true => CycleSeq::Inf(CSeqInf {
                data: self.p1_data,
                hard_dt: None,
            }),
            false => CycleSeq::LoopLimSin(self),
        }
    }
    pub(super) fn optimize_looped(self) -> CycleSeqLooped<T>
    where
        T: Eq,
    {
        match self.p1_data == self.p2_data && self.hard_dt.is_none() {
            true => CycleSeqLooped::Inf(CSeqInf {
                data: self.p1_data,
                hard_dt: None,
            }),
            false => CycleSeqLooped::LoopLimSin(self),
        }
    }
}
impl<T> CSeqLoopLimSin<T>
where
    T: Copy,
{
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::LoopLimSin(*self))
    }
}
