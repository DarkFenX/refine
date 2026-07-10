use super::{seq_limited::CycleSeqLimited, seq_looped::CycleSeqLooped};

pub(in crate::svc) struct CycleSeqSplit<D, HDT> {
    pub(in crate::svc) limited: Option<CycleSeqLimited<D>>,
    pub(in crate::svc) looped: Option<CycleSeqLooped<D, HDT>>,
}
