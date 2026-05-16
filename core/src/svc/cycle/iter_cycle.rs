use super::{
    seq::CycleSeq, seq_inf::CSeqInf, seq_lim::CSeqLim, seq_lim_inf::CSeqLimInf, seq_lim_sin_inf::CSeqLimSinInf,
    seq_loop_lim_sin::CSeqLoopLimSin, traits::GetDuration,
};
use crate::num::{Count, PValue};

////////////////////////////////////////////////////////////////////////////////////////////////////
// High-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleIterItem<T>
where
    T: Copy,
{
    pub(in crate::svc) data: T,
    pub(in crate::svc) time_until_hard_dt: Option<PValue>,
    pub(in crate::svc) hard_dt_duration: Option<PValue>,
}

pub(in crate::svc) enum CycleIter<T>
where
    T: Copy,
{
    Lim(CSeqLimCycleIter<T>),
    Inf(CSeqInfCycleIter<T>),
    LimInf(CSeqLimInfCycleIter<T>),
    LimSinInf(CSeqLimSinInfCycleIter<T>),
    LoopLimSin(CSeqLoopLimSinCycleIter<T>),
}
impl<T> Iterator for CycleIter<T>
where
    T: Copy + GetDuration,
{
    type Item = CycleIterItem<T>;

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

impl<T> CycleSeq<T>
where
    T: Copy + GetDuration,
{
    pub(in crate::svc) fn iter_cycles(&self) -> CycleIter<T> {
        match self {
            Self::Lim(inner) => CycleIter::Lim(CSeqLimCycleIter::new(inner)),
            Self::Inf(inner) => CycleIter::Inf(CSeqInfCycleIter::new(inner)),
            Self::LimInf(inner) => CycleIter::LimInf(CSeqLimInfCycleIter::new(inner)),
            Self::LimSinInf(inner) => CycleIter::LimSinInf(CSeqLimSinInfCycleIter::new(inner)),
            Self::LoopLimSin(inner) => CycleIter::LoopLimSin(CSeqLoopLimSinCycleIter::new(inner)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Lim
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimCycleIter<T>
where
    T: Copy,
{
    item: CycleIterItem<T>,
    repeats_limit: Count,
    // State
    repeats_done: Count,
}
impl<T> CSeqLimCycleIter<T>
where
    T: Copy,
{
    fn new(cseq: &CSeqLim<T>) -> Self {
        Self {
            item: CycleIterItem {
                data: cseq.data,
                time_until_hard_dt: None,
                hard_dt_duration: None,
            },
            repeats_limit: cseq.repeat_count,
            repeats_done: Count::ZERO,
        }
    }
}
impl<T> Iterator for CSeqLimCycleIter<T>
where
    T: Copy,
{
    type Item = CycleIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.repeats_done >= self.repeats_limit {
            return None;
        }
        self.repeats_done += Count::ONE;
        Some(self.item)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inf
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqInfCycleIter<T>
where
    T: Copy,
{
    item: CycleIterItem<T>,
}
impl<T> CSeqInfCycleIter<T>
where
    T: Copy + GetDuration,
{
    fn new(cseq: &CSeqInf<T>) -> Self {
        let time_until_hard_dt = match cseq.hard_dt.is_some() {
            true => Some(cseq.data.get_duration()),
            false => None,
        };
        Self {
            item: CycleIterItem {
                data: cseq.data,
                time_until_hard_dt,
                hard_dt_duration: cseq.hard_dt.map(|v| v.duration),
            },
        }
    }
}
impl<T> Iterator for CSeqInfCycleIter<T>
where
    T: Copy,
{
    type Item = CycleIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.item)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LimInf
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimInfCycleIter<T>
where
    T: Copy,
{
    p1_item: CycleIterItem<T>,
    p1_repeats_limit: Count,
    p2_item: CycleIterItem<T>,
    // State
    p1_repeats_done: Count,
    p1_done: bool,
}
impl<T> CSeqLimInfCycleIter<T>
where
    T: Copy,
{
    fn new(cseq: &CSeqLimInf<T>) -> Self {
        Self {
            p1_item: CycleIterItem {
                data: cseq.p1_data,
                time_until_hard_dt: None,
                hard_dt_duration: None,
            },
            p1_repeats_limit: cseq.p1_repeat_count,
            p2_item: CycleIterItem {
                data: cseq.p2_data,
                time_until_hard_dt: None,
                hard_dt_duration: None,
            },
            p1_repeats_done: Count::ZERO,
            p1_done: false,
        }
    }
}
impl<T> Iterator for CSeqLimInfCycleIter<T>
where
    T: Copy,
{
    type Item = CycleIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.p1_done {
            false => {
                if self.p1_repeats_done >= self.p1_repeats_limit {
                    self.p1_done = true;
                    return Some(self.p2_item);
                }
                self.p1_repeats_done += Count::ONE;
                Some(self.p1_item)
            }
            true => Some(self.p2_item),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LimSinInf
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimSinInfCycleIter<T>
where
    T: Copy,
{
    p1_item: CycleIterItem<T>,
    p1_repeats_limit: Count,
    p2_item: CycleIterItem<T>,
    p3_item: CycleIterItem<T>,
    // State
    p1_repeats_done: Count,
    index: u8,
}
impl<T> CSeqLimSinInfCycleIter<T>
where
    T: Copy,
{
    fn new(cseq: &CSeqLimSinInf<T>) -> Self {
        Self {
            p1_item: CycleIterItem {
                data: cseq.p1_data,
                time_until_hard_dt: None,
                hard_dt_duration: None,
            },
            p1_repeats_limit: cseq.p1_repeat_count,
            p2_item: CycleIterItem {
                data: cseq.p2_data,
                time_until_hard_dt: None,
                hard_dt_duration: None,
            },
            p3_item: CycleIterItem {
                data: cseq.p3_data,
                time_until_hard_dt: None,
                hard_dt_duration: None,
            },
            index: 0,
            p1_repeats_done: Count::ZERO,
        }
    }
}
impl<T> Iterator for CSeqLimSinInfCycleIter<T>
where
    T: Copy,
{
    type Item = CycleIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                if self.p1_repeats_done >= self.p1_repeats_limit {
                    self.index = 1;
                    return Some(self.p2_item);
                }
                self.p1_repeats_done += Count::ONE;
                Some(self.p1_item)
            }
            1 => Some(self.p3_item),
            _ => unreachable!(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LoopLimSin
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLoopLimSinCycleIter<T>
where
    T: Copy,
{
    p1_item_draft: CycleIterItem<T>,
    p1_repeats_limit: Count,
    p2_item: CycleIterItem<T>,
    // State
    p1_repeats_done: Count,
}
impl<T> CSeqLoopLimSinCycleIter<T>
where
    T: Copy + GetDuration,
{
    fn new(cseq: &CSeqLoopLimSin<T>) -> Self {
        let (p2_time_until_hard_dt, hard_dt_duration) = match cseq.hard_dt {
            Some(hard_dt) => (Some(cseq.p2_data.get_duration()), Some(hard_dt.duration)),
            None => (None, None),
        };
        Self {
            p1_item_draft: CycleIterItem {
                data: cseq.p1_data,
                time_until_hard_dt: None,
                hard_dt_duration: None,
            },
            p1_repeats_limit: cseq.p1_repeat_count,
            p2_item: CycleIterItem {
                data: cseq.p2_data,
                time_until_hard_dt: p2_time_until_hard_dt,
                hard_dt_duration,
            },
            p1_repeats_done: Count::ZERO,
        }
    }
}
impl<T> Iterator for CSeqLoopLimSinCycleIter<T>
where
    T: Copy + GetDuration,
{
    type Item = CycleIterItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p1_repeats_done >= self.p1_repeats_limit {
            self.p1_repeats_done = Count::ZERO;
            return Some(self.p2_item);
        }
        let mut p1_item = self.p1_item_draft;
        if self.p2_item.hard_dt_duration.is_some() {
            let p1_duration = self.p1_item_draft.data.get_duration();
            let p1_repeats_left = (self.p1_repeats_limit - self.p1_repeats_done).into_pvalue();
            let p2_duration = self.p2_item.time_until_hard_dt.unwrap();
            p1_item.time_until_hard_dt = Some(p1_duration.mul_add(p1_repeats_left, p2_duration));
        }
        self.p1_repeats_done += Count::ONE;
        Some(p1_item)
    }
}
