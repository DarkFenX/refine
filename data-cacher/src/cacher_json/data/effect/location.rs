#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::cacher_json) enum CEffectLocation {
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
}
impl From<&rc::ad::AEffectLocation> for CEffectLocation {
    fn from(a_effect_location: &rc::ad::AEffectLocation) -> Self {
        match a_effect_location {
            rc::ad::AEffectLocation::Ship => Self::Ship,
            rc::ad::AEffectLocation::Structure => Self::Structure,
            rc::ad::AEffectLocation::Char => Self::Char,
            rc::ad::AEffectLocation::Item => Self::Item,
            rc::ad::AEffectLocation::Other => Self::Other,
            rc::ad::AEffectLocation::Target => Self::Target,
        }
    }
}
impl From<&CEffectLocation> for rc::ad::AEffectLocation {
    fn from(c_effect_location: &CEffectLocation) -> Self {
        match c_effect_location {
            CEffectLocation::Ship => Self::Ship,
            CEffectLocation::Structure => Self::Structure,
            CEffectLocation::Char => Self::Char,
            CEffectLocation::Item => Self::Item,
            CEffectLocation::Other => Self::Other,
            CEffectLocation::Target => Self::Target,
        }
    }
}
