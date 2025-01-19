use crate::sol::svc::calc::SolLocation;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) enum SolLocationKind {
    Ship,
    Structure,
    Character,
}
impl TryFrom<SolLocation> for SolLocationKind {
    type Error = &'static str;

    fn try_from(value: SolLocation) -> Result<Self, Self::Error> {
        match value {
            SolLocation::Ship => Ok(Self::Ship),
            SolLocation::Structure => Ok(Self::Structure),
            SolLocation::Char => Ok(Self::Character),
            _ => Err("unable to convert modifier location to location kind"),
        }
    }
}
