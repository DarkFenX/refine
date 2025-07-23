#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CBuffAggrMode {
    Min,
    Max,
}
impl From<&rc::ad::ABuffAggrMode> for CBuffAggrMode {
    fn from(a_buff_aggr_mode: &rc::ad::ABuffAggrMode) -> Self {
        match a_buff_aggr_mode {
            rc::ad::ABuffAggrMode::Min => Self::Min,
            rc::ad::ABuffAggrMode::Max => Self::Max,
        }
    }
}
impl From<&CBuffAggrMode> for rc::ad::ABuffAggrMode {
    fn from(c_buff_aggr_mode: &CBuffAggrMode) -> Self {
        match c_buff_aggr_mode {
            CBuffAggrMode::Min => Self::Min,
            CBuffAggrMode::Max => Self::Max,
        }
    }
}
