use crate::{util::Named, ReeFloat, ReeInt};

/// An item type - dogma attribute relation.
#[derive(Debug)]
pub struct ItemAttr {
    /// Refers an item type involved in the relation.
    pub item_id: ReeInt,
    /// Refers a dogma attribute involved in the relation.
    pub attr_id: ReeInt,
    /// Value of the attribute.
    pub value: ReeFloat,
}
impl ItemAttr {
    /// Make a new item-attribute relation out of passed data.
    pub fn new(item_id: ReeInt, attr_id: ReeInt, value: ReeFloat) -> Self {
        Self {
            item_id,
            attr_id,
            value,
        }
    }
}
impl Named for ItemAttr {
    fn get_name() -> &'static str {
        "dh::ItemAttr"
    }
}
