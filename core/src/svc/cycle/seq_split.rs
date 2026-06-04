use super::{seq_limited::CycleSeqLimited, seq_looped::CycleSeqLooped};

struct CycleSeqSplit<D, HDT> {
    limited: Option<CycleSeqLimited<D>>,
    looped: Option<CycleSeqLooped<D, HDT>>,
}
