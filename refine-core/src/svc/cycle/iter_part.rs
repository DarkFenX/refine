use super::{
    seq::CycleSeq, seq_var_lim::CSeqLim, seq_var_lim_inf::CSeqLimInf, seq_var_lim_sin_inf::CSeqLimSinInf,
    seq_var_loop_lim_sin::CSeqLoopLimSin, seq_var_loop_sin::CSeqLoopSin,
};
use crate::{
    misc::InfCount,
    num::Count,
    util::{State3, State4},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// High-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CycleSeq<D, HDT> {
    pub(in crate::svc) fn get_parts(&self) -> CSeqParts<'_, D, HDT> {
        let loops = match self {
            Self::Lim(_) | Self::LimInf(_) | Self::LimSinInf(_) => false,
            Self::LoopSin(_) | Self::LoopLimSin(_) => true,
        };
        CSeqParts { cseq: self, loops }
    }
}

pub(crate) struct CSeqParts<'a, D, HDT> {
    cseq: &'a CycleSeq<D, HDT>,
    pub(crate) loops: bool,
}
impl<'a, D, HDT> CSeqParts<'a, D, HDT> {
    pub(crate) fn iter(&self) -> CSeqPartIter<'a, D, HDT> {
        match self.cseq {
            CycleSeq::Lim(inner) => CSeqPartIter::Lim(inner.iter_parts_regular()),
            CycleSeq::LimInf(inner) => CSeqPartIter::LimInf(inner.iter_parts_regular()),
            CycleSeq::LimSinInf(inner) => CSeqPartIter::LimSinInf(inner.iter_parts_regular()),
            CycleSeq::LoopSin(inner) => CSeqPartIter::LoopSin(inner.iter_parts_regular()),
            CycleSeq::LoopLimSin(inner) => CSeqPartIter::LoopLimSin(inner.iter_parts_regular()),
        }
    }
}

pub(in crate::svc) enum CSeqPartIter<'a, D, HDT> {
    Lim(CSeqLimPartIter<'a, D>),
    LimInf(CSeqLimInfPartIter<'a, D>),
    LimSinInf(CSeqLimSinInfPartIter<'a, D>),
    LoopSin(CSeqLoopSinPartIter<'a, D, HDT>),
    LoopLimSin(CSeqLoopLimSinPartIter<'a, D, HDT>),
}
impl<D, HDT> Iterator for CSeqPartIter<'_, D, HDT>
where
    D: Copy,
{
    type Item = CSeqPartInf<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Lim(inner) => inner.next(),
            Self::LoopSin(inner) => inner.next(),
            Self::LimInf(inner) => inner.next(),
            Self::LimSinInf(inner) => inner.next(),
            Self::LoopLimSin(inner) => inner.next(),
        }
    }
}

pub(crate) struct CSeqPartInf<D> {
    pub(crate) data: D,
    pub(crate) repeat_count: InfCount,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Lim
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLim<D> {
    fn iter_parts_regular(&self) -> CSeqLimPartIter<'_, D> {
        CSeqLimPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLimPartIter<'a, D> {
    cseq: &'a CSeqLim<D>,
    yielded: bool,
}
impl<'a, D> CSeqLimPartIter<'a, D> {
    fn new(cseq: &'a CSeqLim<D>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<D> Iterator for CSeqLimPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqPartInf<D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        Some(CSeqPartInf {
            data: self.cseq.data,
            repeat_count: InfCount::Count(self.cseq.repeat_count),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LimInf
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLimInf<D> {
    fn iter_parts_regular(&self) -> CSeqLimInfPartIter<'_, D> {
        CSeqLimInfPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLimInfPartIter<'a, D> {
    cseq: &'a CSeqLimInf<D>,
    state: State3,
}
impl<'a, D> CSeqLimInfPartIter<'a, D> {
    fn new(cseq: &'a CSeqLimInf<D>) -> Self {
        Self {
            cseq,
            state: State3::One,
        }
    }
}
impl<D> Iterator for CSeqLimInfPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqPartInf<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State3::One => {
                self.state = State3::Two;
                Some(CSeqPartInf {
                    data: self.cseq.p1_data,
                    repeat_count: InfCount::Count(self.cseq.p1_repeat_count),
                })
            }
            State3::Two => {
                self.state = State3::Three;
                Some(CSeqPartInf {
                    data: self.cseq.p2_data,
                    repeat_count: InfCount::Infinite,
                })
            }
            State3::Three => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LimSinInf
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D> CSeqLimSinInf<D> {
    fn iter_parts_regular(&self) -> CSeqLimSinInfPartIter<'_, D> {
        CSeqLimSinInfPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLimSinInfPartIter<'a, D> {
    cseq: &'a CSeqLimSinInf<D>,
    state: State4,
}
impl<'a, D> CSeqLimSinInfPartIter<'a, D> {
    fn new(cseq: &'a CSeqLimSinInf<D>) -> Self {
        Self {
            cseq,
            state: State4::One,
        }
    }
}
impl<D> Iterator for CSeqLimSinInfPartIter<'_, D>
where
    D: Copy,
{
    type Item = CSeqPartInf<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State4::One => {
                self.state = State4::Two;
                Some(CSeqPartInf {
                    data: self.cseq.p1_data,
                    repeat_count: InfCount::Count(self.cseq.p1_repeat_count),
                })
            }
            State4::Two => {
                self.state = State4::Three;
                Some(CSeqPartInf {
                    data: self.cseq.p2_data,
                    repeat_count: InfCount::Count(Count::ONE),
                })
            }
            State4::Three => {
                self.state = State4::Four;
                Some(CSeqPartInf {
                    data: self.cseq.p3_data,
                    repeat_count: InfCount::Infinite,
                })
            }
            State4::Four => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LoopSin
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CSeqLoopSin<D, HDT> {
    fn iter_parts_regular(&self) -> CSeqLoopSinPartIter<'_, D, HDT> {
        CSeqLoopSinPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLoopSinPartIter<'a, D, HDT> {
    cseq: &'a CSeqLoopSin<D, HDT>,
    yielded: bool,
}
impl<'a, D, HDT> CSeqLoopSinPartIter<'a, D, HDT> {
    fn new(cseq: &'a CSeqLoopSin<D, HDT>) -> Self {
        Self { cseq, yielded: false }
    }
}
impl<D, HDT> Iterator for CSeqLoopSinPartIter<'_, D, HDT>
where
    D: Copy,
{
    type Item = CSeqPartInf<D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            return None;
        }
        self.yielded = true;
        Some(CSeqPartInf {
            data: self.cseq.data,
            repeat_count: InfCount::Count(Count::ONE),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LoopLimSin
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<D, HDT> CSeqLoopLimSin<D, HDT> {
    fn iter_parts_regular(&self) -> CSeqLoopLimSinPartIter<'_, D, HDT> {
        CSeqLoopLimSinPartIter::new(self)
    }
}

pub(in crate::svc) struct CSeqLoopLimSinPartIter<'a, D, HDT> {
    cseq: &'a CSeqLoopLimSin<D, HDT>,
    state: State3,
}
impl<'a, D, HDT> CSeqLoopLimSinPartIter<'a, D, HDT> {
    fn new(cseq: &'a CSeqLoopLimSin<D, HDT>) -> Self {
        Self {
            cseq,
            state: State3::One,
        }
    }
}
impl<D, HDT> Iterator for CSeqLoopLimSinPartIter<'_, D, HDT>
where
    D: Copy,
{
    type Item = CSeqPartInf<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State3::One => {
                self.state = State3::Two;
                Some(CSeqPartInf {
                    data: self.cseq.p1_data,
                    repeat_count: InfCount::Count(self.cseq.p1_repeat_count),
                })
            }
            State3::Two => {
                self.state = State3::Three;
                Some(CSeqPartInf {
                    data: self.cseq.p2_data,
                    repeat_count: InfCount::Count(Count::ONE),
                })
            }
            State3::Three => None,
        }
    }
}
