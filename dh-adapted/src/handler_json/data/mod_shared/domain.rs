#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CModDomain {
    Ship,
    Structure,
    Char,
    Item,
    Other,
}
impl From<&rc::ad::AModDomain> for CModDomain {
    fn from(mod_domain: &rc::ad::AModDomain) -> Self {
        match mod_domain {
            rc::ad::AModDomain::Ship => Self::Ship,
            rc::ad::AModDomain::Structure => Self::Structure,
            rc::ad::AModDomain::Char => Self::Char,
            rc::ad::AModDomain::Item => Self::Item,
            rc::ad::AModDomain::Other => Self::Other,
        }
    }
}
impl Into<rc::ad::AModDomain> for &CModDomain {
    fn into(self) -> rc::ad::AModDomain {
        match self {
            CModDomain::Ship => rc::ad::AModDomain::Ship,
            CModDomain::Structure => rc::ad::AModDomain::Structure,
            CModDomain::Char => rc::ad::AModDomain::Char,
            CModDomain::Item => rc::ad::AModDomain::Item,
            CModDomain::Other => rc::ad::AModDomain::Other,
        }
    }
}
