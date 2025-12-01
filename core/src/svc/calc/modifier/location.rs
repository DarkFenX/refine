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
impl Location {
    pub(in crate::svc::calc) fn try_from_buff_scope(buff_scope: &AEffectBuffScope) -> Option<Self> {
        match buff_scope {
            AEffectBuffScope::Carrier => None,
            AEffectBuffScope::Projected(item_list_id) => Some(Self::ItemList(*item_list_id)),
            AEffectBuffScope::Fleet(item_list_id) => Some(Self::ItemList(*item_list_id)),
        }
    }
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
