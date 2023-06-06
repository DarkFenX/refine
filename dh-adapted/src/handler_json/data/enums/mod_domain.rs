#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CModDomain {
    Ship,
    Structure,
    Char,
    Item,
    Other,
}
impl From<&rc::consts::ModDomain> for CModDomain {
    fn from(mod_domain: &rc::consts::ModDomain) -> Self {
        match mod_domain {
            rc::consts::ModDomain::Ship => Self::Ship,
            rc::consts::ModDomain::Structure => Self::Structure,
            rc::consts::ModDomain::Char => Self::Char,
            rc::consts::ModDomain::Item => Self::Item,
            rc::consts::ModDomain::Other => Self::Other,
        }
    }
}
impl Into<rc::consts::ModDomain> for &CModDomain {
    fn into(self) -> rc::consts::ModDomain {
        match self {
            CModDomain::Ship => rc::consts::ModDomain::Ship,
            CModDomain::Structure => rc::consts::ModDomain::Structure,
            CModDomain::Char => rc::consts::ModDomain::Char,
            CModDomain::Item => rc::consts::ModDomain::Item,
            CModDomain::Other => rc::consts::ModDomain::Other,
        }
    }
}
