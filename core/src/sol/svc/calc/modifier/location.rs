use crate::ad;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) enum SolLocation {
    Everything,
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
}
impl From<&ad::AEffectLocation> for SolLocation {
    fn from(a_effect_loc: &ad::AEffectLocation) -> Self {
        match a_effect_loc {
            ad::AEffectLocation::Ship => Self::Ship,
            ad::AEffectLocation::Structure => Self::Structure,
            ad::AEffectLocation::Char => Self::Char,
            ad::AEffectLocation::Item => Self::Item,
            ad::AEffectLocation::Other => Self::Other,
            ad::AEffectLocation::Target => Self::Target,
        }
    }
}
impl From<&ad::AEffectBuffScope> for SolLocation {
    fn from(a_buff_scope: &ad::AEffectBuffScope) -> Self {
        match a_buff_scope {
            ad::AEffectBuffScope::Everything => Self::Everything,
            ad::AEffectBuffScope::Ships => Self::Ship,
            ad::AEffectBuffScope::FleetShips => Self::Ship,
        }
    }
}
