use crate::ad::{AEffectBuffScope, AEffectLocation, AItemListId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Location {
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
    ItemList(AItemListId),
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
impl From<&AEffectBuffScope> for Location {
    fn from(buff_scope: &AEffectBuffScope) -> Self {
        Self::ItemList(buff_scope.item_list_id)
    }
}
