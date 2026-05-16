use crate::{
    misc::InfCount,
    num::Count,
    svc::cycle::{CSeqPart, CycleHardDt, CycleSeq, CycleSeqLooped},
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Part 1: runs specified number of times
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqLim<T> {
    pub(in crate::svc) data: T,
    pub(in crate::svc) repeat_count: Count,
}
impl<T> CSeqLim<T> {
    pub(super) fn get_first_cycle(&self) -> &T {
        &self.data
    }
    pub(super) fn get_hard_dt(&self) -> Option<CycleHardDt> {
        None
    }
}
impl<T> CSeqLim<T>
where
    T: Copy,
{
    pub(super) fn iter_cseq_parts_regular(&self) -> CSeqLimPartIter<'_, T> {
        CSeqLimPartIter::new(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> CSeqLim<T> {
    pub(super) fn try_loop_cseq(&self) -> Option<CycleSeqLooped<T>> {
        None
    }
    pub(super) fn convert<U>(self) -> CSeqLim<U>
    where
        U: From<T>,
    {
        CSeqLim {
            data: self.data.into(),
            repeat_count: self.repeat_count,
        }
    }
    pub(in crate::svc) fn convert_with<C, U>(self, converter: &mut C) -> CSeqLim<U>
    where
        C: LibConverter<T, U>,
    {
        CSeqLim {
            data: converter.lib_convert(self.data),
            repeat_count: self.repeat_count,
        }
    }
    pub(in crate::svc) fn optimize(self) -> CycleSeq<T> {
        CycleSeq::Lim(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sequence part iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimPartIter<'a, T> {
    cseq: &'a CSeqLim<T>,
    yielded: bool,
}
impl<'a, T> CSeqLimPartIter<'a, T> {
    fn new(cseq: &'a CSeqLim<T>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<T> Iterator for CSeqLimPartIter<'_, T>
where
    T: Copy,
{
    type Item = CSeqPart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        Some(CSeqPart {
            data: self.cseq.data,
            repeat_count: InfCount::Count(self.cseq.repeat_count),
        })
    }
}
