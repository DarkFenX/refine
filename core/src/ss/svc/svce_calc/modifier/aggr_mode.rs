use crate::{ad, ad::ABuff, defs::AggrKey};

// Defines how a modification will be aggregated.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::svce_calc) enum SsModAggrMode {
    // All modifications are applied.
    Stack,
    // Min value will be used, from values with provided key.
    Min(AggrKey),
    // Max value will be used, from values with provided key.
    Max(AggrKey),
}
impl SsModAggrMode {
    pub(in crate::ss::svc::svce_calc) fn from_a_buff(a_buff: &ABuff) -> Self {
        match a_buff.aggr_mode {
            ad::ABuffAggrMode::Min => Self::Min(a_buff.id),
            ad::ABuffAggrMode::Max => Self::Max(a_buff.id),
        }
    }
}
