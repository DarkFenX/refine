use crate::ss::svc::svce_calc::SsModDomain;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub(in crate::ss::svc::svce_calc) enum SsLocType {
    Ship,
    Structure,
    Character,
}
impl std::convert::TryFrom<SsModDomain> for SsLocType {
    type Error = &'static str;

    fn try_from(value: SsModDomain) -> Result<Self, Self::Error> {
        match value {
            SsModDomain::Ship => Ok(Self::Ship),
            SsModDomain::Structure => Ok(Self::Structure),
            SsModDomain::Char => Ok(Self::Character),
            _ => Err("unable to convert modifier domain to location type"),
        }
    }
}
