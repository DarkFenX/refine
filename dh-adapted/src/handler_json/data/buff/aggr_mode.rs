#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CBuffAggrMode {
    Min,
    Max,
}
impl From<&rc::ad::ABuffAggrMode> for CBuffAggrMode {
    fn from(mod_aggr_mode: &rc::ad::ABuffAggrMode) -> Self {
        match mod_aggr_mode {
            rc::ad::ABuffAggrMode::Min => Self::Min,
            rc::ad::ABuffAggrMode::Max => Self::Max,
        }
    }
}
impl Into<rc::ad::ABuffAggrMode> for &CBuffAggrMode {
    fn into(self) -> rc::ad::ABuffAggrMode {
        match self {
            CBuffAggrMode::Min => rc::ad::ABuffAggrMode::Min,
            CBuffAggrMode::Max => rc::ad::ABuffAggrMode::Max,
        }
    }
}
