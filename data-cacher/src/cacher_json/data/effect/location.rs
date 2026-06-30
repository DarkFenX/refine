use crate::cacher_json::data::AdaptedConv;

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(super) enum CEffectLocation {
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CEffectLocation {
    type AEntity = rc::ad::AEffectLocation;

    fn from_adapted(a_effect_location: &Self::AEntity) -> Self {
        match a_effect_location {
            Self::AEntity::Ship => Self::Ship,
            Self::AEntity::Structure => Self::Structure,
            Self::AEntity::Char => Self::Char,
            Self::AEntity::Item => Self::Item,
            Self::AEntity::Other => Self::Other,
            Self::AEntity::Target => Self::Target,
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::Ship => Self::AEntity::Ship,
            Self::Structure => Self::AEntity::Structure,
            Self::Char => Self::AEntity::Char,
            Self::Item => Self::AEntity::Item,
            Self::Other => Self::AEntity::Other,
            Self::Target => Self::AEntity::Target,
        }
    }
}
