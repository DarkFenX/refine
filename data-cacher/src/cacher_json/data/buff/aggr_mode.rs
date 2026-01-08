#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(super) enum CBuffAggrMode {
    Min,
    Max,
}
impl CBuffAggrMode {
    pub(super) fn from_adapted(a_buff_aggr_mode: &rc::ad::ABuffAggrMode) -> Self {
        match a_buff_aggr_mode {
            rc::ad::ABuffAggrMode::Min => Self::Min,
            rc::ad::ABuffAggrMode::Max => Self::Max,
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::ABuffAggrMode {
        match self {
            Self::Min => rc::ad::ABuffAggrMode::Min,
            Self::Max => rc::ad::ABuffAggrMode::Max,
        }
    }
}
