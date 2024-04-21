#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CEffectDomain {
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
}
impl From<&rc::ad::AEffectDomain> for CEffectDomain {
    fn from(effect_domain: &rc::ad::AEffectDomain) -> Self {
        match effect_domain {
            rc::ad::AEffectDomain::Ship => Self::Ship,
            rc::ad::AEffectDomain::Structure => Self::Structure,
            rc::ad::AEffectDomain::Char => Self::Char,
            rc::ad::AEffectDomain::Item => Self::Item,
            rc::ad::AEffectDomain::Other => Self::Other,
            rc::ad::AEffectDomain::Target => Self::Target,
        }
    }
}
impl Into<rc::ad::AEffectDomain> for &CEffectDomain {
    fn into(self) -> rc::ad::AEffectDomain {
        match self {
            CEffectDomain::Ship => rc::ad::AEffectDomain::Ship,
            CEffectDomain::Structure => rc::ad::AEffectDomain::Structure,
            CEffectDomain::Char => rc::ad::AEffectDomain::Char,
            CEffectDomain::Item => rc::ad::AEffectDomain::Item,
            CEffectDomain::Other => rc::ad::AEffectDomain::Other,
            CEffectDomain::Target => rc::ad::AEffectDomain::Target,
        }
    }
}
