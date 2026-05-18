use super::{
    seq::CycleSeq, seq_inf::CSeqInf, seq_lim::CSeqLim, seq_lim_inf::CSeqLimInf, seq_lim_sin_inf::CSeqLimSinInf,
    seq_loop_lim_sin::CSeqLoopLimSin, traits::GetDuration,
};
use crate::num::{Count, PValue};

////////////////////////////////////////////////////////////////////////////////////////////////////
// High-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CycleSeq<D, HDT>
where
    D: Copy + GetDuration,
    HDT: GetDuration,
{
    pub(in crate::svc) fn iter_cycles(&self) -> CycleIter<D> {
        match self {
            Self::Lim(inner) => CycleIter::Lim(CSeqLimCycleIter::new(inner)),
            Self::Inf(inner) => CycleIter::Inf(CSeqInfCycleIter::new(inner)),
            Self::LimInf(inner) => CycleIter::LimInf(CSeqLimInfCycleIter::new(inner)),
            Self::LimSinInf(inner) => CycleIter::LimSinInf(CSeqLimSinInfCycleIter::new(inner)),
            Self::LoopLimSin(inner) => CycleIter::LoopLimSin(CSeqLoopLimSinCycleIter::new(inner)),
        }
    }
}

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleIterItem<D> {
    pub(in crate::svc) data: D,
    pub(in crate::svc) time_until_hard_dt: Option<PValue>,
    pub(in crate::svc) hard_dt_duration: Option<PValue>,
}

pub(in crate::svc) enum CycleIter<D> {
    Lim(CSeqLimCycleIter<D>),
    Inf(CSeqInfCycleIter<D>),
    LimInf(CSeqLimInfCycleIter<D>),
    LimSinInf(CSeqLimSinInfCycleIter<D>),
    LoopLimSin(CSeqLoopLimSinCycleIter<D>),
}
impl<D> Iterator for CycleIter<D>
where
    D: Copy + GetDuration,
{
    type Item = CycleIterItem<D>;

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
// Lim
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimCycleIter<D> {
    item: CycleIterItem<D>,
    repeats_limit: Count,
    // State
    repeats_done: Count,
}
impl<D> CSeqLimCycleIter<D> {
    fn new(cseq: &CSeqLim<D>) -> Self
    where
        D: Copy,
    {
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
impl<D> Iterator for CSeqLimCycleIter<D>
where
    D: Copy,
{
    type Item = CycleIterItem<D>;

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
pub(in crate::svc) struct CSeqInfCycleIter<D> {
    item: CycleIterItem<D>,
}
impl<D> CSeqInfCycleIter<D> {
    fn new<HDT>(cseq: &CSeqInf<D, HDT>) -> Self
    where
        D: Copy + GetDuration,
        HDT: GetDuration,
    {
        let time_until_hard_dt = match cseq.hard_dt.is_some() {
            true => Some(cseq.data.get_duration()),
            false => None,
        };
        Self {
            item: CycleIterItem {
                data: cseq.data,
                time_until_hard_dt,
                hard_dt_duration: cseq.hard_dt.as_ref().map(|v| v.get_duration()),
            },
        }
    }
}
impl<D> Iterator for CSeqInfCycleIter<D>
where
    D: Copy,
{
    type Item = CycleIterItem<D>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.item)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LimInf
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct CSeqLimInfCycleIter<D> {
    p1_item: CycleIterItem<D>,
    p1_repeats_limit: Count,
    p2_item: CycleIterItem<D>,
    // State
    p1_repeats_done: Count,
    p1_done: bool,
}
impl<D> CSeqLimInfCycleIter<D> {
    fn new(cseq: &CSeqLimInf<D>) -> Self
    where
        D: Copy,
    {
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
impl<D> Iterator for CSeqLimInfCycleIter<D>
where
    D: Copy,
{
    type Item = CycleIterItem<D>;

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
pub(in crate::svc) struct CSeqLimSinInfCycleIter<D> {
    p1_item: CycleIterItem<D>,
    p1_repeats_limit: Count,
    p2_item: CycleIterItem<D>,
    p3_item: CycleIterItem<D>,
    // State
    p1_repeats_done: Count,
    index: u8,
}
impl<D> CSeqLimSinInfCycleIter<D> {
    fn new(cseq: &CSeqLimSinInf<D>) -> Self
    where
        D: Copy,
    {
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
impl<D> Iterator for CSeqLimSinInfCycleIter<D>
where
    D: Copy,
{
    type Item = CycleIterItem<D>;

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
pub(in crate::svc) struct CSeqLoopLimSinCycleIter<D> {
    p1_item_draft: CycleIterItem<D>,
    p1_repeats_limit: Count,
    p2_item: CycleIterItem<D>,
    // State
    p1_repeats_done: Count,
}
impl<D> CSeqLoopLimSinCycleIter<D> {
    fn new<HDT>(cseq: &CSeqLoopLimSin<D, HDT>) -> Self
    where
        D: Copy + GetDuration,
        HDT: GetDuration,
    {
        let (p2_time_until_hard_dt, hard_dt_duration) = match &cseq.hard_dt {
            Some(hard_dt) => (Some(cseq.p2_data.get_duration()), Some(hard_dt.get_duration())),
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
impl<D> Iterator for CSeqLoopLimSinCycleIter<D>
where
    D: Copy + GetDuration,
{
    type Item = CycleIterItem<D>;

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
