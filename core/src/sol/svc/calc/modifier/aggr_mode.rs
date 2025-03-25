use crate::ad;

pub(in crate::sol::svc::calc) type AggrKey = ad::ABuffId;

// Defines how a modification will be aggregated.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) enum AggrMode {
    // All modifications are applied.
    Stack,
    // Min value will be used, from values with provided key.
    Min(AggrKey),
    // Max value will be used, from values with provided key.
    Max(AggrKey),
}
impl AggrMode {
    pub(in crate::sol::svc::calc) fn from_a_buff(a_buff: &ad::ABuff) -> Self {
        match a_buff.aggr_mode {
            ad::ABuffAggrMode::Min => Self::Min(a_buff.id),
            ad::ABuffAggrMode::Max => Self::Max(a_buff.id),
        }
    }
}
