use crate::ad;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::svce_calc) enum SsModDomain {
    Everything,
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
}
impl From<&ad::AEffectDomain> for SsModDomain {
    fn from(a_effect_domain: &ad::AEffectDomain) -> Self {
        match a_effect_domain {
            ad::AEffectDomain::Ship => Self::Ship,
            ad::AEffectDomain::Structure => Self::Structure,
            ad::AEffectDomain::Char => Self::Char,
            ad::AEffectDomain::Item => Self::Item,
            ad::AEffectDomain::Other => Self::Other,
            ad::AEffectDomain::Target => Self::Target,
        }
    }
}
impl From<&ad::AEffectBuffScope> for SsModDomain {
    fn from(a_buff_scope: &ad::AEffectBuffScope) -> Self {
        match a_buff_scope {
            ad::AEffectBuffScope::Everything => Self::Everything,
            ad::AEffectBuffScope::Ships => Self::Ship,
            ad::AEffectBuffScope::FleetShips => Self::Ship,
        }
    }
}
