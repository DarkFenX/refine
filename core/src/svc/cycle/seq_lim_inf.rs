use crate::{
    num::Count,
    svc::cycle::{CycleHardDt, CycleSeq, CycleSeqLooped, seq_inf::CSeqInf},
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
// Part 2: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLimInf<T> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
}
impl<T> CSeqLimInf<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.p1_data
    }
    pub(super) fn get_hard_dt(&self) -> Option<CycleHardDt> {
        None
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqLimInf<T> {
    pub(super) fn convert<U>(self) -> CSeqLimInf<U>
    where
        U: From<T>,
    {
        CSeqLimInf {
            p1_data: U::from(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: U::from(self.p2_data),
        }
    }
    pub(in crate::svc) fn convert_with<C, U>(self, converter: &mut C) -> CSeqLimInf<U>
    where
        C: LibConverter<T, U>,
    {
        CSeqLimInf {
            p1_data: converter.lib_convert(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: converter.lib_convert(self.p2_data),
        }
    }
    pub(in crate::svc) fn optimize(self) -> CycleSeq<T>
    where
        T: Eq,
    {
        match self.p1_data == self.p2_data {
            true => CycleSeq::Inf(CSeqInf {
                data: self.p1_data,
                hard_dt: None,
            }),
            false => CycleSeq::LimInf(self),
        }
    }
}
impl<T> CSeqLimInf<T>
where
    T: Copy,
{
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::Inf(CSeqInf {
            data: self.p2_data,
            hard_dt: None,
        }))
    }
}
