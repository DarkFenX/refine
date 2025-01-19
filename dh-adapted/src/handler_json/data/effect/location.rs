#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CEffectLocation {
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
}
impl From<&rc::ad::AEffectLocation> for CEffectLocation {
    fn from(effect_loc: &rc::ad::AEffectLocation) -> Self {
        match effect_loc {
            rc::ad::AEffectLocation::Ship => Self::Ship,
            rc::ad::AEffectLocation::Structure => Self::Structure,
            rc::ad::AEffectLocation::Char => Self::Char,
            rc::ad::AEffectLocation::Item => Self::Item,
            rc::ad::AEffectLocation::Other => Self::Other,
            rc::ad::AEffectLocation::Target => Self::Target,
        }
    }
}
impl Into<rc::ad::AEffectLocation> for &CEffectLocation {
    fn into(self) -> rc::ad::AEffectLocation {
        match self {
            CEffectLocation::Ship => rc::ad::AEffectLocation::Ship,
            CEffectLocation::Structure => rc::ad::AEffectLocation::Structure,
            CEffectLocation::Char => rc::ad::AEffectLocation::Char,
            CEffectLocation::Item => rc::ad::AEffectLocation::Item,
            CEffectLocation::Other => rc::ad::AEffectLocation::Other,
            CEffectLocation::Target => rc::ad::AEffectLocation::Target,
        }
    }
}
