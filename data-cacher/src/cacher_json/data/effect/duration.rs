#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum CEffectAggroDuration {
    Instant,
    Effect,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CEffectAggroDuration {
    pub(super) fn from_adapted(a_aggro_duration: &rc::ad::AEffectAggroDuration) -> Self {
        match a_aggro_duration {
            rc::ad::AEffectAggroDuration::Instant => Self::Instant,
            rc::ad::AEffectAggroDuration::Effect => Self::Effect,
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::AEffectAggroDuration {
        match self {
            Self::Instant => rc::ad::AEffectAggroDuration::Instant,
            Self::Effect => rc::ad::AEffectAggroDuration::Effect,
        }
    }
}
