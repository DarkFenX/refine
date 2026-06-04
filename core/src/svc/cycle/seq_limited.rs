use super::{seq_var_lim::CSeqLim, seq_var_lim_sin::CSeqLimSin};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) enum CycleSeqLimited<D> {
    Lim(CSeqLim<D>),
    LimSin(CSeqLimSin<D>),
}
