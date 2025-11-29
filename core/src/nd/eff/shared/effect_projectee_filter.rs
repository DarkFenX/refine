use crate::ad::{AAttrId, AItemListId};

// Specifies which items projected effect can target
#[derive(Copy, Clone)]
pub(crate) enum NEffectProjecteeFilter {
    // Valid targets are on item list obtainable with hardcoded ID
    ItemList(AItemListId),
    // Valid targets are on item list, which is specified by this attribute of an item which carries
    // the effect
    ItemListAttr(AAttrId),
}
impl NEffectProjecteeFilter {
    pub(crate) fn get_item_list_attr_id(&self) -> Option<AAttrId> {
        match self {
            Self::ItemList(_) => None,
            Self::ItemListAttr(attr_id) => Some(*attr_id),
        }
    }
}
