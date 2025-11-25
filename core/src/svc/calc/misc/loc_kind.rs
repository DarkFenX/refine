use crate::{svc::calc::Location, ud::UShipKind};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) enum LocationKind {
    Ship,
    Structure,
    Character,
}
impl TryFrom<Location> for LocationKind {
    type Error = &'static str;

    fn try_from(value: Location) -> Result<Self, Self::Error> {
        match value {
            Location::Ship => Ok(Self::Ship),
            Location::Structure => Ok(Self::Structure),
            Location::Char => Ok(Self::Character),
            _ => Err("unable to convert modifier location to location kind"),
        }
    }
}
impl TryFrom<UShipKind> for LocationKind {
    type Error = &'static str;

    fn try_from(value: UShipKind) -> Result<Self, Self::Error> {
        match value {
            UShipKind::Ship => Ok(Self::Ship),
            UShipKind::Structure => Ok(Self::Structure),
            _ => Err("unable to convert ship kind to location kind"),
        }
    }
}
