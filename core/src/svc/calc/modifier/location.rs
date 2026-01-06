use crate::{ad::AEffectLocation, rd::RItemListId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Location {
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
    ItemList(RItemListId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Location {
    pub(in crate::svc::calc) fn from_a_effect_loc(a_effect_loc: AEffectLocation) -> Self {
        match a_effect_loc {
            AEffectLocation::Ship => Self::Ship,
            AEffectLocation::Structure => Self::Structure,
            AEffectLocation::Char => Self::Char,
            AEffectLocation::Item => Self::Item,
            AEffectLocation::Other => Self::Other,
            AEffectLocation::Target => Self::Target,
        }
    }
}
