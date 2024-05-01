use crate::sol::svc::svce_calc::SolDomain;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub(in crate::sol::svc::svce_calc) enum SolLocType {
    Ship,
    Structure,
    Character,
}
impl std::convert::TryFrom<SolDomain> for SolLocType {
    type Error = &'static str;

    fn try_from(value: SolDomain) -> Result<Self, Self::Error> {
        match value {
            SolDomain::Ship => Ok(Self::Ship),
            SolDomain::Structure => Ok(Self::Structure),
            SolDomain::Char => Ok(Self::Character),
            _ => Err("unable to convert modifier domain to location type"),
        }
    }
}
