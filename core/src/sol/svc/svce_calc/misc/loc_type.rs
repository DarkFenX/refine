use crate::sol::svc::svce_calc::SolModDomain;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub(in crate::sol::svc::svce_calc) enum SolLocType {
    Ship,
    Structure,
    Character,
}
impl std::convert::TryFrom<SolModDomain> for SolLocType {
    type Error = &'static str;

    fn try_from(value: SolModDomain) -> Result<Self, Self::Error> {
        match value {
            SolModDomain::Ship => Ok(Self::Ship),
            SolModDomain::Structure => Ok(Self::Structure),
            SolModDomain::Char => Ok(Self::Character),
            _ => Err("unable to convert modifier domain to location type"),
        }
    }
}
