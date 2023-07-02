#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CModDomain {
    Ship,
    Structure,
    Char,
    Item,
    Other,
}
impl From<&rc::ModDomain> for CModDomain {
    fn from(mod_domain: &rc::ModDomain) -> Self {
        match mod_domain {
            rc::ModDomain::Ship => Self::Ship,
            rc::ModDomain::Structure => Self::Structure,
            rc::ModDomain::Char => Self::Char,
            rc::ModDomain::Item => Self::Item,
            rc::ModDomain::Other => Self::Other,
        }
    }
}
impl Into<rc::ModDomain> for &CModDomain {
    fn into(self) -> rc::ModDomain {
        match self {
            CModDomain::Ship => rc::ModDomain::Ship,
            CModDomain::Structure => rc::ModDomain::Structure,
            CModDomain::Char => rc::ModDomain::Char,
            CModDomain::Item => rc::ModDomain::Item,
            CModDomain::Other => rc::ModDomain::Other,
        }
    }
}
