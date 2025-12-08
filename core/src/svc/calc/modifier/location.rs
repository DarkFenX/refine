use crate::{ad::AEffectLocation, rd::RItemListKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Location {
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
    ItemList(RItemListKey),
}
impl From<&AEffectLocation> for Location {
    fn from(effect_loc: &AEffectLocation) -> Self {
        match effect_loc {
            AEffectLocation::Ship => Self::Ship,
            AEffectLocation::Structure => Self::Structure,
            AEffectLocation::Char => Self::Char,
            AEffectLocation::Item => Self::Item,
            AEffectLocation::Other => Self::Other,
            AEffectLocation::Target => Self::Target,
        }
    }
}
