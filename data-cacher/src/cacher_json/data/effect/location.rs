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
impl CEffectLocation {
    pub(super) fn from_adapted(a_effect_location: &rc::ad::AEffectLocation) -> Self {
        match a_effect_location {
            rc::ad::AEffectLocation::Ship => Self::Ship,
            rc::ad::AEffectLocation::Structure => Self::Structure,
            rc::ad::AEffectLocation::Char => Self::Char,
            rc::ad::AEffectLocation::Item => Self::Item,
            rc::ad::AEffectLocation::Other => Self::Other,
            rc::ad::AEffectLocation::Target => Self::Target,
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::AEffectLocation {
        match self {
            Self::Ship => rc::ad::AEffectLocation::Ship,
            Self::Structure => rc::ad::AEffectLocation::Structure,
            Self::Char => rc::ad::AEffectLocation::Char,
            Self::Item => rc::ad::AEffectLocation::Item,
            Self::Other => rc::ad::AEffectLocation::Other,
            Self::Target => rc::ad::AEffectLocation::Target,
        }
    }
}
