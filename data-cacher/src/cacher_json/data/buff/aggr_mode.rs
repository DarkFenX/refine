use crate::cacher_json::data::AdaptedConv;

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(super) enum CBuffAggrMode {
    Min,
    Max,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CBuffAggrMode {
    type AEntity = rc::ad::ABuffAggrMode;

    fn from_adapted(a_buff_aggr_mode: &Self::AEntity) -> Self {
        match a_buff_aggr_mode {
            Self::AEntity::Min => Self::Min,
            Self::AEntity::Max => Self::Max,
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::Min => Self::AEntity::Min,
            Self::Max => Self::AEntity::Max,
        }
    }
}
