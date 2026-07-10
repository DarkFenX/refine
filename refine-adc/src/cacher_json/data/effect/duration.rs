use crate::cacher_json::data::AdaptedConv;

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum CEffectAggroDuration {
    Instant,
    Effect,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CEffectAggroDuration {
    type AEntity = rc::ad::AEffectAggroDuration;

    fn from_adapted(a_aggro_duration: &Self::AEntity) -> Self {
        match a_aggro_duration {
            Self::AEntity::Instant => Self::Instant,
            Self::AEntity::Effect => Self::Effect,
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::Instant => Self::AEntity::Instant,
            Self::Effect => Self::AEntity::Effect,
        }
    }
}
