use crate::ad;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SsModDomain {
    Ship,
    Structure,
    Char,
    Item,
    Other,
}
impl From<&ad::AModDomain> for SsModDomain {
    fn from(a_mod_domain: &ad::AModDomain) -> Self {
        match a_mod_domain {
            ad::AModDomain::Ship => Self::Ship,
            ad::AModDomain::Structure => Self::Structure,
            ad::AModDomain::Char => Self::Char,
            ad::AModDomain::Item => Self::Item,
            ad::AModDomain::Other => Self::Other,
        }
    }
}
