use crate::{
    ad::{ABuffAggrMode, ABuffId},
    rd::RBuff,
};

pub(crate) type AggrKey = ABuffId;

// Defines how a modification will be aggregated.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AggrMode {
    // All modifications are applied.
    Stack,
    // Min value will be used, from values with provided key.
    Min(AggrKey),
    // Max value will be used, from values with provided key.
    Max(AggrKey),
}
impl AggrMode {
    pub(in crate::svc::calc) fn from_buff(buff: &RBuff) -> Self {
        match buff.aggr_mode {
            ABuffAggrMode::Min => Self::Min(buff.id),
            ABuffAggrMode::Max => Self::Max(buff.id),
        }
    }
}
