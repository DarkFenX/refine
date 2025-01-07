use crate::sol::svc::calc::SolDomain;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub(in crate::sol::svc::calc) enum SolLocationKind {
    Ship,
    Structure,
    Character,
}
impl TryFrom<SolDomain> for SolLocationKind {
    type Error = &'static str;

    fn try_from(value: SolDomain) -> Result<Self, Self::Error> {
        match value {
            SolDomain::Ship => Ok(Self::Ship),
            SolDomain::Structure => Ok(Self::Structure),
            SolDomain::Char => Ok(Self::Character),
            _ => Err("unable to convert modifier domain to location kind"),
        }
    }
}
