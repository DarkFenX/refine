use crate::{
    defs::{AttrId, AttrVal, ItemId},
    util::Named,
};

/// An EVE item type-attribute relation.
#[derive(Debug)]
pub struct EItemAttr {
    /// Refers an item type involved in the relation.
    pub item_id: ItemId,
    /// Refers an attribute involved in the relation.
    pub attr_id: AttrId,
    /// Value of the attribute.
    pub value: AttrVal,
}
impl EItemAttr {
    /// Make a new item-attribute relation out of passed data.
    pub fn new(item_id: ItemId, attr_id: AttrId, value: AttrVal) -> Self {
        Self {
            item_id,
            attr_id,
            value,
        }
    }
}
impl Named for EItemAttr {
    fn get_name() -> &'static str {
        "EItemAttr"
    }
}
