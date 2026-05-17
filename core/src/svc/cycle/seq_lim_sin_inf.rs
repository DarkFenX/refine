use crate::{
    num::Count,
    svc::cycle::{CycleHardDt, CycleSeq, CycleSeqLooped, seq_inf::CSeqInf, seq_lim_inf::CSeqLimInf},
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
// Part 2: runs once
// Part 3: repeats infinitely
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLimSinInf<T> {
    pub(in crate::svc) p1_data: T,
    pub(in crate::svc) p1_repeat_count: Count,
    pub(in crate::svc) p2_data: T,
    pub(in crate::svc) p3_data: T,
}
impl<T> CSeqLimSinInf<T> {
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
impl<T> CSeqLimSinInf<T> {
    pub(super) fn convert<U>(self) -> CSeqLimSinInf<U>
    where
        U: From<T>,
    {
        CSeqLimSinInf {
            p1_data: U::from(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: U::from(self.p2_data),
            p3_data: U::from(self.p3_data),
        }
    }
    pub(in crate::svc) fn convert_with<C, U>(self, converter: &mut C) -> CSeqLimSinInf<U>
    where
        C: LibConverter<T, U>,
    {
        CSeqLimSinInf {
            p1_data: converter.lib_convert(self.p1_data),
            p1_repeat_count: self.p1_repeat_count,
            p2_data: converter.lib_convert(self.p2_data),
            p3_data: converter.lib_convert(self.p3_data),
        }
    }
    pub(in crate::svc) fn optimize(self) -> CycleSeq<T>
    where
        T: Eq,
    {
        match (self.p1_data == self.p2_data, self.p2_data == self.p3_data) {
            // Nothing to merge
            (false, false) => CycleSeq::LimSinInf(CSeqLimSinInf {
                p1_data: self.p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data: self.p2_data,
                p3_data: self.p3_data,
            }),
            // Merge part 2 into tail
            (false, true) => CycleSeq::LimInf(CSeqLimInf {
                p1_data: self.p1_data,
                p1_repeat_count: self.p1_repeat_count,
                p2_data: self.p3_data,
            }),
            // Merge part 2 into head
            (true, false) => CycleSeq::LimInf(CSeqLimInf {
                p1_data: self.p1_data,
                p1_repeat_count: self.p1_repeat_count + Count::ONE,
                p2_data: self.p3_data,
            }),
            // Whole sequence becomes a simple infinity
            (true, true) => CycleSeq::Inf(CSeqInf {
                data: self.p1_data,
                hard_dt: None,
            }),
        }
    }
}
impl<T> CSeqLimSinInf<T>
where
    T: Copy,
{
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        Some(CycleSeqLooped::Inf(CSeqInf {
            data: self.p3_data,
            hard_dt: None,
        }))
    }
}
