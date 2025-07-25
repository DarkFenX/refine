use crate::{ad, rd, util::GetId};

pub(crate) type AggrKey = ad::ABuffId;

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
    pub(in crate::svc::calc) fn from_r_buff(r_buff: &rd::RBuff) -> Self {
        match r_buff.get_aggr_mode() {
            ad::ABuffAggrMode::Min => Self::Min(r_buff.get_id()),
            ad::ABuffAggrMode::Max => Self::Max(r_buff.get_id()),
        }
    }
}
